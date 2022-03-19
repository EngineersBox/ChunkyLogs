use self::log_entry::LogEntry;

pub struct LogGroup<'a> {
    pub entries: Vec<&'a LogEntry>;
}