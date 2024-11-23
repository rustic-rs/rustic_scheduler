//! RusticScheduler Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use std::collections::HashMap;

use anyhow::{bail, Result};
use cron::Schedule;
use rustic_backend::BackendOptions;
use rustic_core::{BackupOptions, RepositoryOptions, SnapshotOptions};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct RusticSchedulerConfig {
    pub global: GlobalOptions,
    pub repository: AllRepositoryOptions,
    pub clientgroup: HashMap<String, ClientGroupOptions>,
    #[serde_as(as = "HashMap<_,DisplayFromStr>")]
    pub schedules: HashMap<String, Schedule>,
    pub options: HashMap<String, AllBackupOptions>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct GlobalOptions {
    pub address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ClientGroupOptions {
    pub clients: Vec<String>,
    pub sources: Vec<SourceOptions>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct SourceOptions {
    pub source: String,
    pub schedule: String,
    pub options: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct AllBackupOptions {
    #[serde(flatten)]
    pub backup_opts: BackupOptions,
    #[serde(flatten)]
    pub snapshot_opts: SnapshotOptions,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct AllRepositoryOptions {
    #[serde(flatten)]
    pub be: BackendOptions,
    #[serde(flatten)]
    pub repo: RepositoryOptions,
}

impl RusticSchedulerConfig {
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
