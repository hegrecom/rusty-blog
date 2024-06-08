use axum::{
    extract::{rejection::JsonRejection, Path, State},
    Json,
};

use crate::{
    core::{
        error::Error,
        pageable::Pageable,
        success_response::{SuccessResponse, SuccessResponseBuilder},
    },
    post::{
        dto::post, repository::post_repository::PostRepository,
        service::post_fetch_service::PostFetchService,
    },
};

pub async fn index(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    pageable: Result<Json<Pageable>, JsonRejection>,
) -> Result<SuccessResponse, Error> {
    match pageable {
        Ok(Json(pageable)) => {
            let page_response = PostFetchService::new(post_repository(pool.clone()))
                .fetch_list(pageable, Some(post::Status::Published))
                .await?;
            Ok(SuccessResponseBuilder::new()
                .data(page_response.items())
                .meta(page_response.page_meta())
                .build())
        }
        Err(err) => Err(Error::BadRequest(err.to_string())),
    }
}

pub async fn show(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(post_id): Path<i32>,
) -> Result<SuccessResponse, Error> {
    let post = PostFetchService::new(post_repository(pool.clone()))
        .fetch(post_id, Some(post::Status::Published))
        .await?;
    Ok(SuccessResponseBuilder::new().data(post).build())
}

fn post_repository(pool: deadpool_diesel::postgres::Pool) -> PostRepository {
    PostRepository::new(pool.clone())
}
