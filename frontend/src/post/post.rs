use chrono::{DateTime, TimeZone};

#[derive(Clone)]
pub struct Post<Tz: TimeZone> {
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Tz>,
    pub updated_at: DateTime<Tz>,
}
