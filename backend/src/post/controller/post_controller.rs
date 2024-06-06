use axum::{
    extract::{rejection::JsonRejection, State},
    Json,
};

use crate::{
    core::{
        error::Error,
        success_response::{SuccessResponse, SuccessResponseBuilder},
    },
    post::{dto::post_request::PostRequest, service::post_creation_service::PostCreationService},
};

pub async fn create(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    post_request: Result<Json<PostRequest>, JsonRejection>,
) -> Result<SuccessResponse, Error> {
    match post_request {
        Ok(Json(post_request)) => {
            let post = PostCreationService::new(pool).create(post_request).await?;
            Ok(SuccessResponseBuilder::new().data(post).build())
        }
        Err(err) => Err(Error::BadRequest(err.to_string())),
    }
}
