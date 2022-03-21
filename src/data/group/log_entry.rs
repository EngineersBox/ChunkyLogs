use chrono::{DateTime, Utc};

pub enum LogAction {
    CREATE,
    UPDATE,
    INSERT,
    DELETE,
}

pub struct LogEntry<'a> {
    pub timestamp: DateTime<Utc>,
    pub action: LogAction,
    pub target: &'a str,
    pub desc: &'a str,
}