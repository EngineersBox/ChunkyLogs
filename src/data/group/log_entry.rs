use std::string::FromUtf8Error;
use chrono::{DateTime, NaiveDateTime, Utc};
use crate::data::chunk::chunk::Byte;
use crate::data::datetime::datetime_utils::epoch_to_datetime;
use crate::data::group::exceptions::group_exceptions;

#[derive(Clone)]
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

const MAX_ENTRIES_PER_GROUP: usize = 1000;
const MIN_LOG_ENTRY_SIZE: usize = 10; // Ex: "<8 bytes timestamp><1 byte action><n bytes target><n bytes message>\0x00"
const LOG_ENTRY_TIMESTAMP_OFFSET: usize = 0;
const LOG_ENTRY_ACTION_OFFSET: usize = LOG_ENTRY_TIMESTAMP_OFFSET + 8;
const LOG_ENTRY_TARGET_OFFSET: usize = LOG_ENTRY_ACTION_OFFSET + 1;

#[derive(Clone)]
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
    pub(crate) fn process_entry(data: &[Byte]) -> Result<(LogEntry, usize), group_exceptions::GroupEntryProcessingError> {
        if data.len() < MIN_LOG_ENTRY_SIZE {
            return Err(group_exceptions::GroupEntryProcessingError{
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
        log_entry.timestamp = epoch_to_datetime(u64::from_ne_bytes(timestamp_bytes));

        log_entry.action = LogAction::from(data[LOG_ENTRY_ACTION_OFFSET]);

        if data.len() == LOG_ENTRY_TARGET_OFFSET {
            return Ok((log_entry, LOG_ENTRY_TARGET_OFFSET));
        } else if data.len() == LOG_ENTRY_TARGET_OFFSET + 1 {
            return match data[LOG_ENTRY_TARGET_OFFSET] {
                0x00 => Ok((log_entry, LOG_ENTRY_TARGET_OFFSET + 1)),
                byteVal => Err(group_exceptions::GroupEntryProcessingError{
                    message: format!(
                        "Invalid log entry terminator byte. Expected: 0x00, got: {:#04x}",
                        byteVal
                    ),
                })
            }
        }
        let mut idx: usize = LOG_ENTRY_TARGET_OFFSET;
        match LogEntry::process_bytes_to_string(LOG_ENTRY_TARGET_OFFSET, data) {
            Err(e) => return Err(group_exceptions::GroupEntryProcessingError{
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
        match LogEntry::process_bytes_to_string(idx, data) {
            Err(e) => return Err(group_exceptions::GroupEntryProcessingError{
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
}