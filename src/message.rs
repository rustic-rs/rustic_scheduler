use crate::{config::AllRepositoryOptions, scheduler::ClientStats};
use anyhow::Result;
use rustic_core::{repofile::SnapshotFile, BackupOptions, SnapshotOptions};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Serialize, Deserialize)]
pub struct HandshakeMessage {
    pub client: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "result")]
pub enum HandshakeResultMessage {
    Ok,
    Error { message: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupMessage {
    pub repo_opts: AllRepositoryOptions,
    pub backup_opts: BackupOptions,
    pub snapshot_opts: SnapshotOptions,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "result")]
pub enum BackupResultMessage {
    Ok { snapshot: Box<SnapshotFile> },
    Error { message: String },
}

pub(crate) enum ClientMessage {
    Backup { client: String, msg: BackupMessage },
}

pub(crate) enum NotifyMessage {
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
