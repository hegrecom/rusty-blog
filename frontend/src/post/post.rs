use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
