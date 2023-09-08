use anyhow::{bail, Result};
use chrono::{DateTime, Local};
use cron::Schedule;
use rustic_core::Id;
use std::{cmp::Ordering, collections::HashMap, time::Duration};

const MAX_WAIT_TIME: Duration = Duration::from_secs(3600);
type Time = DateTime<Local>;

pub enum ClientState {
    NotConnected,
    Idle,
    Processing(Time),
}

pub struct Client {
    state: ClientState,
    sources: Vec<Source>, // ordered by next_invocation!
}

impl Client {
    pub fn new() -> Self {
        Self {
            state: ClientState::NotConnected,
            sources: Vec::new(),
        }
    }

    fn connect(&mut self) -> Result<()> {
        match self.state {
            ClientState::NotConnected => {
                self.state = ClientState::Idle;
                Ok(())
            }
            _ => bail!("client is already connected"),
        }
    }

    fn disconnect(&mut self) {
        if let ClientState::Idle = self.state {
            self.state = ClientState::NotConnected;
        }
    }

    fn next_invocation(&self) -> Option<Time> {
        match self.state {
            ClientState::Idle | ClientState::NotConnected => {
                self.sources.get(0).map(|s| s.next_invocation).flatten()
            }
            ClientState::Processing(_) => None,
        }
    }

    fn to_processing(&mut self, time: Time) -> (String, ClientState) {
        println!("backing up {}", self.sources[0].source);
        let state = std::mem::replace(&mut self.state, ClientState::Processing(time));
        (self.sources[0].source.clone(), state)
    }

    fn log_not_connected(&mut self, time: Time) {
        // TODO: Check ClientState
        let source = &mut self.sources[0];
        source.history.push(SourceBackup {
            scheduled: source.next_invocation.unwrap(),
            real: time,
            status: SourceBackupStatus::NotConnected,
        });
        source.update_invocation(time);
        self.state = ClientState::NotConnected;
        self.sort_sources();
    }

    fn to_idle(&mut self, time: Time, status: SourceBackupStatus) {
        if let ClientState::Processing(start_time) = self.state {
            let source = &mut self.sources[0];
            source.last_success = source.next_invocation;
            source.history.push(SourceBackup {
                scheduled: source.next_invocation.unwrap(),
                real: start_time,
                status,
            });
            self.state = ClientState::Idle;
            source.update_invocation(time);
            self.sort_sources();
        }
        // TODO handle other ClientStates => Error!
    }

    fn sort_sources(&mut self) {
        self.sources
            .sort_unstable_by(|s1, s2| match (s1.next_invocation, s2.next_invocation) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(s1), Some(s2)) => s1.cmp(&s2),
            })
    }

    pub fn add_source(&mut self, source: Source) {
        self.sources.push(source);
        self.sort_sources();
    }
}

pub struct Source {
    source: String,
    schedule: Schedule,
    next_invocation: Option<Time>,
    last_success: Option<Time>,
    history: Vec<SourceBackup>,
}

pub struct SourceBackup {
    scheduled: Time,
    real: Time,
    status: SourceBackupStatus,
}

pub enum SourceBackupStatus {
    Ok(Id),
    Error(String),
    NotConnected,
}

impl Source {
    pub fn new(source: String, schedule: Schedule) -> Self {
        let mut source = Self {
            source,
            schedule,
            last_success: None,
            next_invocation: None,
            history: Vec::new(),
        };
        source.update_invocation(Local::now());
        source
    }

    fn update_invocation(&mut self, after: Time) {
        self.next_invocation = self.schedule.after_owned(after).next();
    }
}

struct NextAction(Option<(String, Time)>);

impl NextAction {
    fn min_with(&mut self, name: &String, date: Option<Time>) {
        if let Some(date) = date {
            match self.0 {
                Some((_, cur_date)) if date >= cur_date => {}
                _ => {
                    self.0 = Some((name.clone(), date));
                }
            }
        }
    }
    fn date(&self) -> Option<Time> {
        self.0.as_ref().map(|(_, date)| *date)
    }
}

pub struct Clients {
    clients: HashMap<String, Client>,
    next_action: NextAction,
}

impl Clients {
    pub fn new() -> Self {
        let clients = HashMap::new();
        Self {
            clients,
            next_action: NextAction(None),
        }
    }

    fn compute_next_action(&mut self) {
        let next_action = self
            .clients
            .iter()
            .fold(NextAction(None), |mut acc, (name, client)| {
                acc.min_with(&name, client.next_invocation());
                acc
            });
        self.next_action = next_action;
    }

    pub fn add_client(&mut self, name: String, client: Client) {
        self.next_action.min_with(&name, client.next_invocation());
        self.clients.insert(name, client);
    }

    pub fn wait_time(&self, time: Time) -> Result<Duration> {
        let time = match self.next_action.date() {
            None => MAX_WAIT_TIME,
            Some(n) => (n - time)
                .max(chrono::Duration::zero())
                .to_std()?
                .min(MAX_WAIT_TIME),
        };
        Ok(time)
    }

    pub fn process_next(&mut self, time: Time) -> Option<(String, String)> {
        let result = match &self.next_action.0 {
            None => None,
            Some((_, next_date)) if next_date > &time => None,
            Some((next_client, _)) => {
                println!("backup from {next_client}");
                let client = self.clients.get_mut(next_client).unwrap();
                match client.state {
                    ClientState::Processing(_) => {
                        println!(
                            "this shouldn't happen -> client {next_client} is already processing..."
                        );
                        None
                    }
                    ClientState::NotConnected => {
                        client.log_not_connected(time);
                        None
                    }
                    ClientState::Idle => {
                        let (source, _) = client.to_processing(time);
                        Some((next_client.clone(), source))
                    }
                }
            }
        };
        self.compute_next_action();
        result
    }

    pub fn finish_process(&mut self, client: String, time: Time, status: SourceBackupStatus) {
        self.clients.get_mut(&client).unwrap().to_idle(time, status);
        self.compute_next_action();
    }

    pub fn connect_client(&mut self, client: String) -> Result<()> {
        match self.clients.get_mut(&client) {
            None => {
                bail!("client {client} is not configured");
            }
            Some(client) => client.connect()?,
        }
        self.compute_next_action();
        Ok(())
    }

    pub fn disconnect_client(&mut self, client: String) {
        self.clients
            .get_mut(&client)
            .map(|client| client.disconnect());
        self.compute_next_action();
    }
}
