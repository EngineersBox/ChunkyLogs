use super::log_entry::LogEntry;

pub struct LogGroup<'a> {
    pub entries: Vec<&'a LogEntry<'a>>,
}