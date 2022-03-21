use chrono::{DateTime, NaiveDateTime, Utc};
use crate::data::chunk::chunk::{Chunk, ChunkCompressionState, Byte};
use crate::data::datetime::datetime_utils::epoch_to_datetime;
use crate::data::group::exceptions::group_exceptions;

use super::log_entry::LogEntry;

const MAX_ENTRIES_PER_GROUP: usize = 1000;
const MIN_LOG_ENTRY_SIZE: usize = 10; // Ex: "<8 bytes timestamp><1 byte action><n bytes target><n bytes message>\0x00"
static LOG_ENTRY_SEPARATOR: &'static str = "";

const LOG_ENTRY_TIMESTAMP_OFFSET: usize = 0;
const LOG_ENTRY_ACTION_OFFSET: usize = LOG_ENTRY_TIMESTAMP_OFFSET + 8;
const LOG_ENTRY_TARGET_OFFSET: usize = LOG_ENTRY_ACTION_OFFSET + 1;

pub struct LogGroup {
    pub ts_from: DateTime<Utc>,
    pub ts_to: DateTime<Utc>,
    pub entries: Vec<LogEntry>,
}

impl LogGroup {
    pub fn new() -> LogGroup {
        LogGroup {
            ts_from: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            ts_to: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            entries: Vec::with_capacity(MAX_ENTRIES_PER_GROUP),
        }
    }
    pub fn append_entry(&mut self, entry: &LogEntry) -> Result<(), group_exceptions::GroupEntryAppendError> {
        if self.entries.len() >= MAX_ENTRIES_PER_GROUP {
            return Err(group_exceptions::GroupEntryAppendError{
                message: format!("Maximum number of entries reached {}", MAX_ENTRIES_PER_GROUP)
            });
        }
        if self.entries.is_empty() {
            self.ts_from = entry.timestamp;
        }
        self.ts_to = entry.timestamp;
        self.entries.push((*entry).clone());
        return Ok(());
    }
    pub fn from_chunk(chunk: &Chunk) -> Result<LogGroup, group_exceptions::GroupChunkProcessingError> {
        if chunk.state == ChunkCompressionState::COMPRESSED {
            return Err(group_exceptions::GroupChunkProcessingError{
                message: "Chunk is not in DECOMPRESSED state".to_string()
            });
        } else if chunk.length <= 0 {
            return Err(group_exceptions::GroupChunkProcessingError{
                message: "No chunk data to process".to_string()
            });
        }
        let mut log_group: LogGroup = LogGroup::new();
        log_group.ts_from = epoch_to_datetime(chunk.ts_from);
        log_group.ts_to = epoch_to_datetime(chunk.ts_to);

        let mut idx: usize = 0;
        loop {
            if idx >= chunk.data.len() {
                break;
            }
            match LogEntry::process_entry(&chunk.data.as_slice()[idx..]) {
                Err(e) => return Err(group_exceptions::GroupChunkProcessingError{
                    message: e.message,
                }),
                Ok((log_entry, i)) => {
                    log_group.entries.push(log_entry);
                    idx = i;
                },
            };
        }
        return Ok(LogGroup::new());
    }
}
