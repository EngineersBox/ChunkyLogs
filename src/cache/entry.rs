use crate::group::log_entry::LogEntry;

pub struct CacheEntry {
    pub entry: LogEntry;
    pub ttl: u32;
}