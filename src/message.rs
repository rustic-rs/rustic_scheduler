use rustic_core::{repofile::SnapshotFile, BackupOptions, RepositoryOptions, SnapshotOptions};
use serde::{Deserialize, Serialize};

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
    pub repo_opts: RepositoryOptions,
    pub backup_opts: BackupOptions,
    pub snapshot_opts: SnapshotOptions,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "result")]
pub enum BackupResultMessage {
    Ok { snapshot: SnapshotFile },
    Error { message: String },
}
