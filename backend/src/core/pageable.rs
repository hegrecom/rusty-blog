use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pageable {
    page: i64,
    per: i64,
    sort: Option<String>,
}

impl Pageable {
    pub fn new(page: i64, per: i64, sort: Option<String>) -> Self {
        Self { page, per, sort }
    }

    pub fn page(&self) -> i64 {
        self.page
    }

    pub fn per(&self) -> i64 {
        self.per
    }

    pub fn sort(&self) -> Option<&str> {
        (&self.sort).as_deref()
    }

    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.per
    }

    pub fn limit(&self) -> i64 {
        self.per
    }
}
