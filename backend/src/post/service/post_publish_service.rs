use crate::{
    core::error::Error,
    post::{dto::post::Post, repository::post_repository::PostRepository},
};

pub struct PostPublishService {
    post_repository: PostRepository,
}

impl PostPublishService {
    pub fn new(post_repository: PostRepository) -> Self {
        Self { post_repository }
    }

    pub async fn publish(&self, id: i32) -> Result<Post, Error> {
        let post = self.post_repository.publish_post(id).await?;
        Ok(post)
    }
}
