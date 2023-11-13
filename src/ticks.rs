use chrono::prelude::*;

pub fn current_ticks() -> i64 {
    let utc_now = Utc::now();
    utc_now.timestamp_millis()
}
