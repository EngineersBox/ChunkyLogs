use super::entry::CacheEntry;

pub struct Cache<'a> {
    pub entries: Vec<CacheEntry<'a>>
}