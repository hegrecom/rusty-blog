use crate::{
    core::error::Error,
    post::{
        dto::{post::Post, post_creation::PostCreation},
        repository::post_repository::PostRepository,
    },
};

pub struct PostCreationService {
    pool: deadpool_diesel::postgres::Pool,
}

impl PostCreationService {
    pub fn new(pool: deadpool_diesel::postgres::Pool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, post_creation: PostCreation) -> Result<Post, Error> {
        let post_respository = PostRepository::new(self.pool.clone());
        let post = post_respository.create(post_creation).await?;
        Ok(post)
    }
}
