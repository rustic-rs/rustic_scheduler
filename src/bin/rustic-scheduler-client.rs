use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use gethostname::gethostname;
use log::{info, warn};
use rustic_core::{repofile::SnapshotFile, PathList, Repository};
use tungstenite::{connect, Message};
use url::Url;

use rustic_scheduler::message::{BackupMessage, BackupResultMessage, HandshakeMessage};

#[derive(clap_derive::Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[clap(long)]
    /// Set client name. Default: hostname
    name: Option<String>,

    /// Server websocket URL to connect to, e.g. ws://host:3012/ws
    server: Url,
}

fn main() {
    env_logger::init();
    let opts = Opts::parse();
    let name = opts
        .name
        .unwrap_or_else(|| gethostname().to_string_lossy().to_string());

    // TODO: retry with backoff
    loop {
        if let Err(err) = connect_client(opts.server.clone(), name.clone()) {
            eprintln!("{err}");
            warn!("error {err}, retrying...");
            // retry conneting after 5s
            sleep(Duration::from_secs(5));
        }
    }
}

fn connect_client(server: Url, name: String) -> Result<()> {
    let (mut socket, _) = connect(server.as_str())?;

    info!("Connected to the server");
    println!("Connected to the server");

    // handshake
    let handshake_msg = HandshakeMessage { client: name };
    let handshake_msg = serde_json::to_string(&handshake_msg)?;
    socket.send(handshake_msg.into())?;

    loop {
        let msg = socket.read()?;

        match msg {
            Message::Ping(..) => socket.send(Message::Pong(Vec::new()))?,
            _ => {
                let msg = msg.into_data();
                let backup_msg: BackupMessage = serde_json::from_slice(&msg)?;

                let snap_msg = match do_backup(backup_msg) {
                    Ok(snap) => {
                        println!("{snap:?}");
                        BackupResultMessage::Ok {
                            snapshot: Box::new(snap),
                        }
                    }
                    Err(err) => BackupResultMessage::Error {
                        message: err.to_string(),
                    },
                };
                let snap_msg = serde_json::to_string(&snap_msg)?;
                socket.send(snap_msg.into())?;
            }
        }
    }
    // socket.close(None);
}

fn do_backup(message: BackupMessage) -> Result<SnapshotFile> {
    let backends = message.repo_opts.be.to_backends()?;

    let repo_opts = message.repo_opts.repo;

    let repo = Repository::new(&repo_opts, &backends)?
        .open()?
        .to_indexed_ids()?;

    let source = PathList::from_string(&message.source)?.sanitize()?;

    let snap = message.snapshot_opts.to_snapshot()?;

    let snap = repo.backup(&message.backup_opts, &source, snap)?;

    Ok(snap)
}
