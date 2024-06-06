use axum::{
    extract::{rejection::JsonRejection, Path, State},
    Json,
};

use crate::{
    core::{
        error::Error,
        success_response::{SuccessResponse, SuccessResponseBuilder},
    },
    post::{
        dto::post_request::PostRequest,
        repository::post_repository::PostRepository,
        service::{
            post_creation_service::PostCreationService, post_update_service::PostUpdateService,
        },
    },
};

pub async fn create(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    post_request: Result<Json<PostRequest>, JsonRejection>,
) -> Result<SuccessResponse, Error> {
    match post_request {
        Ok(Json(post_request)) => {
            let post = PostCreationService::new(post_repository(pool.clone()))
                .create(post_request)
                .await?;
            Ok(SuccessResponseBuilder::new().data(post).build())
        }
        Err(err) => Err(Error::BadRequest(err.to_string())),
    }
}

pub async fn update(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(post_id): Path<i32>,
    post_request: Result<Json<PostRequest>, JsonRejection>,
) -> Result<SuccessResponse, Error> {
    match post_request {
        Ok(Json(post_request)) => {
            let post = PostUpdateService::new(post_repository(pool.clone()))
                .update(post_id, post_request)
                .await?;

            Ok(SuccessResponseBuilder::new().data(post).build())
        }
        Err(err) => Err(Error::BadRequest(err.to_string())),
    }
}

fn post_repository(pool: deadpool_diesel::postgres::Pool) -> PostRepository {
    PostRepository::new(pool.clone())
}
