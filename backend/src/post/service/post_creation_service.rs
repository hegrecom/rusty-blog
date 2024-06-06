use crate::{
    core::error::Error,
    post::{
        dto::{post::Post, post_request::PostRequest},
        repository::post_repository::PostRepository,
    },
};

pub struct PostCreationService {
    post_repository: PostRepository,
}

impl PostCreationService {
    pub fn new(post_repository: PostRepository) -> Self {
        Self { post_repository }
    }

    pub async fn create(&self, post_request: PostRequest) -> Result<Post, Error> {
        let post = self.post_repository.create(post_request).await?;
        Ok(post)
    }
}
