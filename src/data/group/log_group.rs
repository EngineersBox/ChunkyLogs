use std::fmt;
use crate::data::group::exceptions::group_exceptions;
use super::log_entry::LogEntry;

const MAX_ENTRIES_PER_GROUP: usize = 1000;

pub struct LogGroup<'a> {
    pub entries: Vec<&'a LogEntry<'a>>,
}

impl<'a> LogGroup<'a> {
    pub fn new() -> LogGroup<'a> {
        LogGroup {
            entries: Vec::with_capacity(MAX_ENTRIES_PER_GROUP),
        }
    }
    pub fn append_entry(&mut self, entry: &'a LogEntry) -> Result<(), group_exceptions::GroupEntryAppendError> {
        if self.entries.len() >= MAX_ENTRIES_PER_GROUP {
            return Err(group_exceptions::GroupEntryAppendError{
                message: format!("Maximum number of entries reached {}", MAX_ENTRIES_PER_GROUP)
            });
        }
        self.entries.push(entry);
        return Ok(());
    }
}