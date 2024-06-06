use diesel::{RunQueryDsl, SelectableHelper};

use crate::{
    core::error::Error,
    post::dto::{post::Post, post_creation::PostCreation},
    schema,
};

pub struct PostRepository {
    pool: deadpool_diesel::postgres::Pool,
}

impl PostRepository {
    pub fn new(pool: deadpool_diesel::postgres::Pool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, post_creation: PostCreation) -> Result<Post, Error> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|err| Error::InternalServerError(err.to_string()))?;
        let result = conn
            .interact(|conn| {
                diesel::insert_into(schema::posts::table)
                    .values(post_creation)
                    .returning(Post::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(|err| Error::InternalServerError(err.to_string()))?
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        Ok(result)
    }
}
