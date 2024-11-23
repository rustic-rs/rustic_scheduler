//! `client` subcommand
use std::thread::sleep;
use std::time::Duration;

use abscissa_core::{
    config::Override, status_err, Application, Command, FrameworkError, Runnable, Shutdown,
};
use anyhow::Result;
use clap::Parser;
use gethostname::gethostname;
use log::{info, warn};
use tungstenite::{connect, Message};
use url::Url;

use rustic_core::{repofile::SnapshotFile, PathList, Repository};

use crate::{
    config::RusticSchedulerConfig,
    message::{BackupMessage, BackupResultMessage, HandshakeMessage},
    prelude::RUSTIC_SCHEDULER_APP,
};

/// `client` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(Command, Debug, Parser)]
pub struct ClientCmd {
    /// Set client name. Default: hostname
    #[clap(short)]
    name: Option<String>,

    /// Server websocket URL to connect to, e.g. ws://host:3012/ws
    #[clap(long)]
    url: Url,
}

impl Override<RusticSchedulerConfig> for ClientCmd {
    fn override_config(
        &self,
        config: RusticSchedulerConfig,
    ) -> std::result::Result<RusticSchedulerConfig, FrameworkError> {
        // TODO - override config with CLI settings

        Ok(config)
    }
}

impl Runnable for ClientCmd {
    /// Start the application.
    fn run(&self) {
        let res = || -> Result<()> {
            let name = self
                .name
                .clone()
                .unwrap_or_else(|| gethostname().to_string_lossy().to_string());

            // TODO: retry with backoff
            loop {
                if let Err(err) = connect_client(self.url.clone(), name.clone()) {
                    eprintln!("{err}");
                    warn!("error {err}, retrying...");
                    // retry conneting after 5s
                    sleep(Duration::from_secs(5));
                }
            }
        };

        if let Err(err) = res() {
            status_err!("{}", err);
            RUSTIC_SCHEDULER_APP.shutdown(Shutdown::Crash);
        };
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
