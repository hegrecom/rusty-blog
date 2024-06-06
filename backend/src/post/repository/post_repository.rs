use deadpool_diesel::postgres::Object;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    core::error::Error,
    post::dto::{
        post::{self, Post},
        post_request::PostRequest,
    },
    schema,
};

pub struct PostRepository {
    pool: deadpool_diesel::postgres::Pool,
}

impl PostRepository {
    pub fn new(pool: deadpool_diesel::postgres::Pool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, post_request: PostRequest) -> Result<Post, Error> {
        let conn = self.get_db_connection().await?;
        let result = conn
            .interact(|conn| {
                diesel::insert_into(schema::posts::table)
                    .values(post_request)
                    .returning(Post::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(|err| Error::InternalServerError(err.to_string()))?
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        Ok(result)
    }

    pub async fn update(&self, id: i32, post_request: PostRequest) -> Result<Post, Error> {
        let conn = self.get_db_connection().await?;
        let result = conn
            .interact(move |conn| {
                diesel::update(schema::posts::table.find(id))
                    .set((
                        post_request,
                        schema::posts::updated_at.eq(chrono::Utc::now().naive_utc()),
                    ))
                    .get_result(conn)
            })
            .await
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        match result {
            Ok(post) => Ok(post),
            Err(err) => match err {
                diesel::result::Error::NotFound => Err(Error::RecordNotFound),
                _ => Err(Error::InternalServerError(err.to_string())),
            },
        }
    }

    pub async fn publish_post(&self, id: i32) -> Result<Post, Error> {
        let conn = self.get_db_connection().await?;
        let result = conn
            .interact(move |conn| {
                diesel::update(schema::posts::table.find(id))
                    .set((
                        schema::posts::status.eq(post::Status::Published),
                        schema::posts::updated_at.eq(chrono::Utc::now().naive_utc()),
                    ))
                    .get_result(conn)
            })
            .await
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        match result {
            Ok(post) => Ok(post),
            Err(err) => match err {
                diesel::result::Error::NotFound => Err(Error::RecordNotFound),
                _ => Err(Error::InternalServerError(err.to_string())),
            },
        }
    }

    pub async fn fetch(&self, offset: i64, limit: i64) -> Result<Vec<Post>, Error> {
        let conn = self.get_db_connection().await?;
        let result = conn
            .interact(move |conn| Self::pagable_select_query(offset, limit).load::<Post>(conn))
            .await
            .map_err(|err| Error::InternalServerError(err.to_string()))?
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        Ok(result)
    }

    pub async fn total_count(&self) -> Result<i64, Error> {
        let conn = self.get_db_connection().await?;
        let result = conn
            .interact(move |conn| schema::posts::table.count().get_result(conn))
            .await
            .map_err(|err| Error::InternalServerError(err.to_string()))?
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        Ok(result)
    }

    pub async fn fetch_by_status(
        &self,
        status: post::Status,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Post>, Error> {
        let conn = self.get_db_connection().await?;
        let result = conn
            .interact(move |conn| {
                Self::pagable_select_query(offset, limit)
                    .filter(schema::posts::status.eq(status))
                    .load::<Post>(conn)
            })
            .await
            .map_err(|err| Error::InternalServerError(err.to_string()))?
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        Ok(result)
    }

    pub async fn total_count_by_status(&self, status: post::Status) -> Result<i64, Error> {
        let conn = self.get_db_connection().await?;
        let result = conn
            .interact(move |conn| {
                schema::posts::table
                    .filter(schema::posts::status.eq(status))
                    .count()
                    .get_result(conn)
            })
            .await
            .map_err(|err| Error::InternalServerError(err.to_string()))?
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        Ok(result)
    }

    pub async fn find(&self, id: i32) -> Result<Post, Error> {
        let conn = self.get_db_connection().await?;
        let result = conn
            .interact(move |conn| schema::posts::table.find(id).get_result(conn))
            .await
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        match result {
            Ok(post) => Ok(post),
            Err(err) => match err {
                diesel::result::Error::NotFound => Err(Error::RecordNotFound),
                _ => Err(Error::InternalServerError(err.to_string())),
            },
        }
    }

    pub async fn delete(&self, id: i32) -> Result<(), Error> {
        let conn = self.get_db_connection().await?;
        let result = conn
            .interact(move |conn| diesel::delete(schema::posts::table.find(id)).execute(conn))
            .await
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        match result {
            Ok(1) => Ok(()),
            Ok(_) => Err(Error::RecordNotFound),
            Err(err) => Err(Error::InternalServerError(err.to_string())),
        }
    }

    async fn get_db_connection(&self) -> Result<Object, Error> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|err| Error::InternalServerError(err.to_string()))?;

        Ok(conn)
    }

    fn pagable_select_query(
        offset: i64,
        limit: i64,
    ) -> schema::posts::BoxedQuery<'static, diesel::pg::Pg> {
        schema::posts::table
            .order(schema::posts::id.desc())
            .offset(offset)
            .limit(limit)
            .into_boxed()
    }
}
