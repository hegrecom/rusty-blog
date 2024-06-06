use axum::{
    extract::{rejection::JsonRejection, State},
    Json,
};

use crate::{
    core::{
        error::Error,
        success_response::{SuccessResponse, SuccessResponseBuilder},
    },
    post::{dto::post_creation::PostCreation, repository::post_repository::PostRepository},
};

pub async fn create(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    post_creation: Result<Json<PostCreation>, JsonRejection>,
) -> Result<SuccessResponse, Error> {
    match post_creation {
        Ok(Json(post_creation)) => {
            let post_respository = PostRepository::new(pool.clone());
            let post = post_respository.create(post_creation).await?;
            Ok(SuccessResponseBuilder::new().data(post).build())
        }
        Err(err) => Err(Error::BadRequest(err.to_string())),
    }
}
