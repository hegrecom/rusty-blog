use crate::{
    core::{error::Error, page_response::PageResponse, pageable::Pageable},
    post::{
        dto::post::{Post, Status},
        repository::post_repository::PostRepository,
    },
};

pub struct PostFetchService {
    post_repository: PostRepository,
}

impl PostFetchService {
    pub fn new(post_repository: PostRepository) -> Self {
        Self { post_repository }
    }

    pub async fn fetch(&self, id: i32, status: Option<Status>) -> Result<Post, Error> {
        let post = match status {
            Some(status) => self.post_repository.find_by_status(id, status).await?,
            None => self.post_repository.find(id).await?,
        };
        Ok(post)
    }

    pub async fn fetch_list(
        &self,
        pageable: Pageable,
        status: Option<Status>,
    ) -> Result<PageResponse<Post>, Error> {
        let (posts, count) = match status {
            Some(status) => {
                let posts = self
                    .post_repository
                    .fetch_by_status(status, pageable.offset(), pageable.limit())
                    .await?;
                let count = self.post_repository.total_count_by_status(status).await?;
                (posts, count)
            }
            None => {
                let posts = self
                    .post_repository
                    .fetch(pageable.offset(), pageable.limit())
                    .await?;
                let count = self.post_repository.total_count().await?;
                (posts, count)
            }
        };

        Ok(PageResponse::new(posts, count, pageable))
    }
}
