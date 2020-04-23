use chrono::{NaiveDateTime, DateTime, Utc};

pub struct UTCClock;

impl UTCClock {

    pub fn now_at_utc(&self) -> NaiveDateTime {
        Utc::now().naive_utc()
    }

}