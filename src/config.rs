use std::collections::HashMap;

use anyhow::{bail, Result};
use cron::Schedule;
use rustic_core::{BackupOptions, RepositoryOptions, SnapshotOptions};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[derive(Clone, Deserialize)]
pub struct GlobalOptions {
    pub address: String,
}

#[derive(Clone, Deserialize)]
pub struct ClientGroupOptions {
    pub clients: Vec<String>,
    pub sources: Vec<SourceOptions>,
}

#[derive(Clone, Deserialize)]
pub struct SourceOptions {
    pub source: String,
    pub schedule: String,
    pub options: String,
}

#[derive(Clone, Deserialize)]
pub struct AllBackupOptions {
    #[serde(flatten)]
    pub backup_opts: BackupOptions,
    #[serde(flatten)]
    pub snapshot_opts: SnapshotOptions,
}

#[serde_as]
#[derive(Clone, Deserialize)]
pub struct ConfigFile {
    pub global: GlobalOptions,
    pub repository: RepositoryOptions,
    pub clientgroup: HashMap<String, ClientGroupOptions>,
    #[serde_as(as = "HashMap<_,DisplayFromStr>")]
    pub schedules: HashMap<String, Schedule>,
    pub options: HashMap<String, AllBackupOptions>,
}

impl ConfigFile {
    pub fn validate(&self) -> Result<()> {
        for (name, cg) in self.clientgroup.iter() {
            for source in &cg.sources {
                if !self.options.contains_key(&source.options) {
                    bail!(
                        "Clientgroup {name}, Source {}: Options {} are undefined!",
                        source.source,
                        source.options
                    );
                }
                if !self.schedules.contains_key(&source.schedule) {
                    bail!(
                        "Clientgroup {name}, Source {}: Scheduler {} is undefined!",
                        source.source,
                        source.schedule
                    );
                }
            }
        }

        Ok(())
    }
}
