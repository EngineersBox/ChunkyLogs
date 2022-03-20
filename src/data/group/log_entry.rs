pub enum LogAction {
    CREATE,
    UPDATE,
    INSERT,
    DELETE,
}

pub struct LogEntry<'a> {
    pub timestamp: u32,
    pub action: LogAction,
    pub target: &'a str,
    pub desc: &'a str,
}