use axum::{
    extract::{rejection::JsonRejection, State},
    Json,
};

use crate::{
    admin::{dto::login_request::LoginRequest, repository::admin_repository::AdminRepository},
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
        Ok(login_request) => {
            let password = login_request.password();
            let admin = AdminRepository::new(pool.clone())
                .fetch_admin()
                .await
                .unwrap();
            match admin.authenticate(password) {
                Ok(true) => Ok(SuccessResponseBuilder::new().build()),
                Ok(false) => Err(Error::Unauthorized),
                Err(err) => Err(Error::InternalServerError(format!("{}", err.to_string()))),
            }
        }
        Err(err) => Err(Error::BadRequest(err.body_text())),
    }
}
