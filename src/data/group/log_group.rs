use std::string::FromUtf8Error;
use std::time::{Duration, UNIX_EPOCH};
use chrono::{DateTime, NaiveDateTime, Utc};
use crate::data::chunk::chunk::{Chunk, ChunkCompressionState, Byte};
use crate::data::group::exceptions::group_exceptions;

use super::log_entry::{LogEntry, LogAction};

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
    fn epoch_to_datetime(millis: u64) -> DateTime::<Utc> {
        return DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_millis(millis));
    }
    fn process_bytes_to_string(start_idx: usize, data: &[Byte]) -> Result<(String, usize), FromUtf8Error> {
        let mut idx = start_idx;
        let mut next_byte: Byte;
        let mut buffer: Vec<Byte> = Vec::new();
        loop {
            next_byte = data[idx];
            idx += 1;
            if next_byte == 0x00 {
                break;
            }
            buffer.push(next_byte);
        }
        return match String::from_utf8(buffer) {
            Err(e) => Err(e),
            Ok(s) => Ok((s, idx)),
        };
    }
    fn process_entry(data: &[Byte]) -> Result<(LogEntry, usize), group_exceptions::GroupChunkProcessingError> {
        if data.len() < MIN_LOG_ENTRY_SIZE {
            return Err(group_exceptions::GroupChunkProcessingError{
                message: format!(
                    "Could not process entry with size smaller than minimum. Expected: {}, got: {}",
                    MIN_LOG_ENTRY_SIZE,
                    data.len(),
                )
            });
        }
        let mut log_entry: LogEntry = LogEntry::new();

        let mut timestamp_bytes: [Byte; 8] = [0; 8];
        timestamp_bytes.copy_from_slice(&data[LOG_ENTRY_TIMESTAMP_OFFSET..LOG_ENTRY_ACTION_OFFSET]);
        log_entry.timestamp = LogGroup::epoch_to_datetime(u64::from_ne_bytes(timestamp_bytes));

        log_entry.action = LogAction::from(data[LOG_ENTRY_ACTION_OFFSET]);

        if data.len() == LOG_ENTRY_TARGET_OFFSET {
            return Ok((log_entry, LOG_ENTRY_TARGET_OFFSET));
        } else if data.len() == LOG_ENTRY_TARGET_OFFSET + 1 {
            return match data[LOG_ENTRY_TARGET_OFFSET] {
                0x00 => Ok((log_entry, LOG_ENTRY_TARGET_OFFSET + 1)),
                byteVal => Err(group_exceptions::GroupChunkProcessingError{
                    message: format!(
                        "Invalid log entry terminator byte. Expected: 0x00, got: {:#04x}",
                        byteVal
                    ),
                })
            }
        }
        let mut idx: usize = LOG_ENTRY_TARGET_OFFSET;
        match LogGroup::process_bytes_to_string(LOG_ENTRY_TARGET_OFFSET, data) {
            Err(e) => return Err(group_exceptions::GroupChunkProcessingError{
                message: format!(
                    "Target was not a valid UTF-8 string sequence: {}",
                    e.to_string(),
                ),
            }),
            Ok((s, i)) => {
                log_entry.target = s;
                idx = i;
            },
        };
        if data.len() == idx {
            return Ok((log_entry, idx));
        }
        match LogGroup::process_bytes_to_string(idx, data) {
            Err(e) => return Err(group_exceptions::GroupChunkProcessingError{
                message: format!(
                    "Message was not a valid UTF-8 string sequence: {}",
                    e.to_string(),
                ),
            }),
            Ok((s, i)) => {
                log_entry.target = s;
                idx = i;
            },
        };
        return Ok((log_entry, idx));
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
        log_group.ts_from = LogGroup::epoch_to_datetime(chunk.ts_from);
        log_group.ts_to = LogGroup::epoch_to_datetime(chunk.ts_to);

        let mut idx: usize = 0;
        loop {
            if idx >= chunk.data.len() {
                break;
            }
            match LogGroup::process_entry(&chunk.data.as_slice()[idx..]) {
                Err(e) => return Err(e),
                Ok((log_entry, i)) => {
                    log_group.entries.push(log_entry);
                    idx = i;
                },
            };
        }
        return Ok(LogGroup::new());
    }
}
