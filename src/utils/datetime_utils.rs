use std::time::{Duration, UNIX_EPOCH};
use chrono::{DateTime, Utc};

pub fn epoch_to_datetime(millis: u64) -> DateTime::<Utc> {
    return DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_millis(millis));
}