use anyhow::Result;
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::{Html, Response},
    routing::get,
    Router,
};
use chrono::Local;
use log::warn;
use sailfish::TemplateOnce;
use std::{collections::HashMap, fs::read_to_string, time::Duration};
use tokio::{
    net::TcpListener,
    spawn,
    sync::{mpsc, oneshot},
    time::sleep,
};

use rustic_scheduler::config::{AllBackupOptions, ConfigFile};
use rustic_scheduler::message::{BackupMessage, BackupResultMessage, HandshakeMessage};
use rustic_scheduler::scheduler::{Client, ClientStats, Clients, Source, SourceBackupStatus};

enum ClientMessage {
    Backup { client: String, msg: BackupMessage },
}

enum NotifyMessage {
    Connect {
        client: String,
        channel: mpsc::Sender<ClientMessage>,
    },
    Disconnect {
        client: String,
    },
    BackupResult {
        client: String,
        msg: BackupResultMessage,
    },
    StatsRequest {
        client: String,
        channel: oneshot::Sender<Result<ClientStats>>,
    },
}

#[tokio::main]
async fn main() {
    // FIXME: Unhardcode config file and expose via CLI settings
    let config = read_to_string("./config/rustic_scheduler.toml").unwrap();
    let config: ConfigFile = toml::from_str(&config).unwrap();
    config.validate().unwrap();

    // Add clients from config file to scheduler
    let mut options_mapper = HashMap::new();
    let mut clients = Clients::new();
    for (_, cg) in config.clientgroup.iter() {
        for name in &cg.clients {
            let mut client = Client::new();
            for source in &cg.sources {
                client.add_source(Source::new(
                    source.source.clone(),
                    config.schedules[&source.schedule].clone(),
                ));
                options_mapper.insert(
                    (name.clone(), source.source.clone()),
                    source.options.clone(),
                );
            }
            clients.add_client(name.clone(), client);
        }
    }

    let (wtx, mut rx) = mpsc::channel(1);

    // The backup loop handling the schedules
    spawn(async move {
        let mut client_channels: HashMap<String, mpsc::Sender<ClientMessage>> = HashMap::new();
        let sleep_timer = sleep(Duration::ZERO);
        tokio::pin!(sleep_timer);

        loop {
            tokio::select! {
                _ = &mut sleep_timer => {
                    if let Some((client, source)) = clients.process_next(Local::now()) {
                        let repo_opts = config.repository.clone();
                        let AllBackupOptions {
                            backup_opts,
                            snapshot_opts,
                        } = config.options[&options_mapper[&(client.clone(), source.clone())]]
                            .clone();
                        let msg = BackupMessage {
                            repo_opts,
                            backup_opts,
                            snapshot_opts,
                            source,
                        };
                        client_channels.get(&client).unwrap().send(ClientMessage::Backup { client, msg }).await.unwrap();
                    }
                }
                Some(res) = rx.recv() => {
                    match res {
                        NotifyMessage::BackupResult{client, msg:BackupResultMessage::Ok {snapshot} } => {
                            println!("backup to {client}, {} finished successfully. Got snapshot {}", snapshot.paths, snapshot.id);
                            clients.finish_process(client, Local::now(), SourceBackupStatus::Ok(snapshot.id));
                        }
                        NotifyMessage::BackupResult{client, msg:BackupResultMessage::Error {message} } => {
                            println!("backup to {client} failed: {}", message);
                            clients.finish_process(client, Local::now(), SourceBackupStatus::Error(message));
                        }
                        NotifyMessage::Connect{client, channel} => {
                            if let Err(err) =  clients.connect_client(client.clone()){
                                eprintln!("Error: {err}, continuing...");
                                warn!("Error: {err}, continuing...");
                            } else {
                                println!("client {client} connected.");
                                client_channels.insert(client, channel);
                            }
                        }
                        NotifyMessage::Disconnect{client} => {
                            println!("reading websocket failed; disconnect client {client}");
                            client_channels.remove(&client);
                            clients.disconnect_client(client);
                        }
                        NotifyMessage::StatsRequest{client, channel} => {
                            channel.send(clients.client_stats(client)).unwrap();
                        }
                    }
                }
            }

            let wait_time = clients.wait_time(Local::now()).unwrap_or_else(|err| {
                warn!("Error determining wait time: {err}");
                Duration::from_secs(5)
            });
            println!("waiting {wait_time:?}");
            sleep_timer.set(sleep(wait_time));
        }
    });

    // build our application with a single route
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/client/:client", get(client_handler))
        .with_state(wtx);

    // run it with hyper on localhost:3012
    let listener = TcpListener::bind(&config.global.address).await.unwrap();

    println!("Listening on http://{}", config.global.address);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn client_handler(
    Path(client): Path<String>,
    State(wtx): State<mpsc::Sender<NotifyMessage>>,
) -> Html<String> {
    let (tx, wrx) = oneshot::channel();

    wtx.send(NotifyMessage::StatsRequest {
        client,
        channel: tx,
    })
    .await
    .unwrap();

    let stats = wrx.await.unwrap().unwrap();
    Html(stats.render_once().unwrap())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(wtx): State<mpsc::Sender<NotifyMessage>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, wtx))
}

async fn handle_socket(mut socket: WebSocket, wtx: mpsc::Sender<NotifyMessage>) {
    let (tx, mut wrx) = mpsc::channel(1);

    // handshake
    let handshake_msg = socket.recv().await.unwrap().unwrap().into_data();
    let handshake_msg: HandshakeMessage = serde_json::from_slice(&handshake_msg).unwrap();
    let client_name = handshake_msg.client;
    println!("client {client_name} wants to connected.");
    wtx.send(NotifyMessage::Connect {
        client: client_name.clone(),
        channel: tx,
    })
    .await
    .unwrap();

    loop {
        tokio::select! {
            msg = socket.recv() => {
                match msg {
                    None | Some(Err(_)) => {
                        wtx.send(NotifyMessage::Disconnect { client: client_name.clone() }).await.unwrap();
                    }
                    Some(Ok(_)) => {
                        // ignore message
                    }
                }

            }
            msg = wrx.recv() => {
                match msg {
                    Some(ClientMessage::Backup{ client, msg}) =>  {
                        let data = serde_json::to_string(&msg).unwrap();
                        if let Err(err) = socket.send(data.into()).await {
                            println!("writing websocket failed; disconnect client {client}: {err}");
                            break;
                        }
                        println!("waiting for backup to {client}, {} to finish...", msg.source);

                        match socket.recv().await {
                            Some(Ok(result)) => {
                                let result: BackupResultMessage =
                                    serde_json::from_slice(&result.into_data()).unwrap();
                                wtx.send(NotifyMessage::BackupResult { client: client.clone(), msg: result }).await.unwrap();
                            }
                            Some(Err(err)) => {
                                println!("reading websocket failed; disconnect client {client}: {err}");
                                break;
                            }
                            None => {
                                println!("client {client} disconnected");
                                break;
                            }
                        }
                    }
                    None => {
                        println!("client {client_name} disconnected");
                                break;
                    }
                }

            }
        };
        wtx.send(NotifyMessage::Disconnect {
            client: client_name.clone(),
        })
        .await
        .unwrap();
    }
}
