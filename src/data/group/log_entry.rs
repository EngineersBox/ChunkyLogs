use chrono::{DateTime, NaiveDateTime, Utc};

pub enum LogAction {
    CREATE,
    UPDATE,
    INSERT,
    DELETE,
}

impl From<u8> for LogAction {
    fn from(idx: u8) -> LogAction {
        match idx {
            0 => LogAction::CREATE,
            1 => LogAction::UPDATE,
            2 => LogAction::INSERT,
            3 => LogAction::DELETE,
            _ => LogAction::CREATE,
        }
    }
}

impl Into<u8> for LogAction {
    fn into(self) -> u8 {
        match self {
            LogAction::CREATE => 0,
            LogAction::UPDATE => 1,
            LogAction::INSERT => 2,
            LogAction::DELETE => 3,
        }
    }
}

pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub action: LogAction,
    pub target: String,
    pub desc: String,
}

impl LogEntry {
    pub fn new() -> LogEntry {
        LogEntry {
            timestamp: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            action: LogAction::CREATE,
            target: String::new(),
            desc: String::new(),
        }
    }
}