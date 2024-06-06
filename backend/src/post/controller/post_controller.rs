use axum::{
    extract::{rejection::JsonRejection, State},
    Json,
};

use crate::{
    core::{
        error::Error,
        success_response::{SuccessResponse, SuccessResponseBuilder},
    },
    post::{dto::post_creation::PostCreation, service::post_creation_service::PostCreationService},
};

pub async fn create(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    post_creation: Result<Json<PostCreation>, JsonRejection>,
) -> Result<SuccessResponse, Error> {
    match post_creation {
        Ok(Json(post_creation)) => {
            let post = PostCreationService::new(pool).create(post_creation).await?;
            Ok(SuccessResponseBuilder::new().data(post).build())
        }
        Err(err) => Err(Error::BadRequest(err.to_string())),
    }
}
