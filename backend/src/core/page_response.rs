use super::{page_meta::PageMeta, pageable::Pageable};

#[derive(Debug)]
pub struct PageResponse<T> {
    items: Vec<T>,
    total: i64,
    pageable: Pageable,
}

impl<T> PageResponse<T> {
    pub fn new(items: Vec<T>, total: i64, pageable: Pageable) -> Self {
        Self {
            items,
            total,
            pageable,
        }
    }

    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    pub fn total(&self) -> i64 {
        self.total
    }

    pub fn pageable(&self) -> &Pageable {
        &self.pageable
    }
}

impl<T> From<PageResponse<T>> for PageMeta {
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
