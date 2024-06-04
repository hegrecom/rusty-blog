use axum::{
    extract::{rejection::JsonRejection, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    admin::{dto::login_request::LoginRequest, repository::admin_repository::AdminRepository},
    core::error::Error,
};

pub async fn login(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    login_request: Result<Json<LoginRequest>, JsonRejection>,
) -> Result<Response, Error> {
    match login_request {
        Ok(login_request) => {
            let password = login_request.password();
            let admin = AdminRepository::new(pool.clone())
                .fetch_admin()
                .await
                .unwrap();
            match admin.authenticate(password) {
                Ok(true) => {
                    return Ok((StatusCode::OK, format!("Hello, {}!", password)).into_response())
                }
                Ok(false) => Err(Error::Unauthorized),
                Err(err) => Err(Error::InternalServerError(format!("{:?}", err))),
            }
        }
        Err(err) => Err(Error::BadRequest(err.body_text())),
    }
}
