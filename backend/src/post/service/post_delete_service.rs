use crate::{core::error::Error, post::repository::post_repository::PostRepository};

pub struct PostDeleteService {
    post_repository: PostRepository,
}

impl PostDeleteService {
    pub fn new(post_repository: PostRepository) -> Self {
        Self { post_repository }
    }

    pub async fn delete(&self, id: i32) -> Result<(), Error> {
        self.post_repository.delete(id).await?;
        Ok(())
    }
}
