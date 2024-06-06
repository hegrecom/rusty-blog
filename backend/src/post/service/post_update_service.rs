use crate::{
    core::error::Error,
    post::{
        dto::{post::Post, post_request::PostRequest},
        repository::post_repository::PostRepository,
    },
};

pub struct PostUpdateService {
    post_repository: PostRepository,
}

impl PostUpdateService {
    pub fn new(post_repository: PostRepository) -> Self {
        Self { post_repository }
    }

    pub async fn update(&self, id: i32, post_request: PostRequest) -> Result<Post, Error> {
        let post = self.post_repository.update(id, post_request).await?;
        Ok(post)
    }
}
