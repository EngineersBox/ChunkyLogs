use chrono::{DateTime, NaiveDateTime, Utc};
use crate::data::chunk::chunk::Chunk;
use crate::data::group::exceptions::group_exceptions;
use super::log_entry::LogEntry;

const MAX_ENTRIES_PER_GROUP: usize = 1000;

pub struct LogGroup<'a> {
    pub ts_from: DateTime<Utc>,
    pub ts_to: DateTime<Utc>,
    pub entries: Vec<&'a LogEntry<'a>>,
}

impl<'a> LogGroup<'a> {
    pub fn new() -> LogGroup<'a> {
        LogGroup {
            ts_from: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            ts_to: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            entries: Vec::with_capacity(MAX_ENTRIES_PER_GROUP),
        }
    }
    pub fn append_entry(&mut self, entry: &'a LogEntry) -> Result<(), group_exceptions::GroupEntryAppendError> {
        if self.entries.len() >= MAX_ENTRIES_PER_GROUP {
            return Err(group_exceptions::GroupEntryAppendError{
                message: format!("Maximum number of entries reached {}", MAX_ENTRIES_PER_GROUP)
            });
        }
        if self.entries.is_empty() {
            self.ts_from = entry.timestamp;
        }
        self.ts_to = entry.timestamp;
        self.entries.push(entry);
        return Ok(());
    }
    pub fn from_chunk(chunk: &Chunk) -> Result<LogGroup<'a>, group_exceptions::GroupChunkProcessingError> {
        return Ok(LogGroup::<'a>::new());
    }
}