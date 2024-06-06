use serde::Serialize;

use super::into_json::IntoJson;

#[derive(Debug, Serialize)]
pub struct PageMeta {
    page: i64,
    per: i64,
    total_pages: i64,
    total: i64,
}

impl PageMeta {
    pub fn new(page: i64, per: i64, total_pages: i64, total: i64) -> Self {
        Self {
            page,
            per,
            total_pages,
            total,
        }
    }
}

impl IntoJson for PageMeta {}
