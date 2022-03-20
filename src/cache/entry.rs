use crate::data::group::log_entry::LogEntry;

pub struct CacheEntry<'a> {
    pub entry: LogEntry<'a>,
    pub ttl: u32,
}