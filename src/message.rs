use rustic_core::{repofile::SnapshotFile, BackupOptions, SnapshotOptions};
use serde::{Deserialize, Serialize};

use crate::config::AllRepositoryOptions;

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
