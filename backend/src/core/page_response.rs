use std::ops::{Deref, DerefMut};

use serde::Serialize;

use super::{into_json::IntoJson, page_meta::PageMeta, pageable::Pageable};

#[derive(Debug, Serialize)]
pub struct Items<T: Serialize>(Vec<T>);

impl<T> From<Vec<T>> for Items<T>
where
    T: Serialize,
{
    fn from(items: Vec<T>) -> Self {
        Self(items)
    }
}

impl<T> IntoJson for Items<T> where T: Serialize {}

impl<T> IntoJson for &Items<T> where T: Serialize {}

impl<T> Deref for Items<T>
where
    T: Serialize,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Items<T>
where
    T: Serialize,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct PageResponse<T: Serialize> {
    items: Items<T>,
    total: i64,
    pageable: Pageable,
}

impl<T> PageResponse<T>
where
    T: Serialize,
{
    pub fn new(items: Vec<T>, total: i64, pageable: Pageable) -> Self {
        Self {
            items: items.into(),
            total,
            pageable,
        }
    }

    pub fn items(&self) -> &Items<T> {
        (&self.items).into()
    }

    pub fn total(&self) -> i64 {
        self.total
    }

    pub fn pageable(&self) -> &Pageable {
        &self.pageable
    }

    pub fn page_meta(self) -> PageMeta {
        self.into()
    }
}

impl<T> From<PageResponse<T>> for PageMeta
where
    T: Serialize,
{
    fn from(page_response: PageResponse<T>) -> Self {
        let total_pages =
            (page_response.total() as f64 / page_response.pageable().per() as f64).ceil() as i64;
        Self::new(
            page_response.pageable().page(),
            page_response.pageable().per(),
            total_pages,
            page_response.total(),
        )
    }
}
