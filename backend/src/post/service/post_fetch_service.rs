use crate::{
    core::error::Error,
    post::{dto::post::Post, repository::post_repository::PostRepository},
};

pub struct PostFetchService {
    post_repository: PostRepository,
}

impl PostFetchService {
    pub fn new(post_repository: PostRepository) -> Self {
        Self { post_repository }
    }

    pub async fn fetch(&self, id: i32) -> Result<Post, Error> {
        let post = self.post_repository.find(id).await?;
        Ok(post)
    }
}
