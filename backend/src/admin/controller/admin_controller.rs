use axum::{
    extract::{rejection::JsonRejection, State},
    Json,
};

use crate::{
    admin::{
        dto::login_request::LoginRequest, service::authentication_service::AuthenticationService,
    },
    core::{
        error::Error,
        success_response::{SuccessResponse, SuccessResponseBuilder},
    },
};

pub async fn login(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    login_request: Result<Json<LoginRequest>, JsonRejection>,
) -> Result<SuccessResponse, Error> {
    match login_request {
        Ok(Json(login_request)) => {
            let authentication_service = AuthenticationService::new(pool.clone());
            let token = authentication_service.authenticate(login_request).await?;

            Ok(SuccessResponseBuilder::new().data(token).build())
        }
        Err(err) => Err(Error::BadRequest(err.body_text())),
    }
}
