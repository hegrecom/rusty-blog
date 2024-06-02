use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::admin::dto::login_request::LoginRequest;

pub async fn login(Json(login_request): Json<LoginRequest>) -> Response {
    let password = login_request.password();

    (StatusCode::OK, format!("Hello, {}!", password)).into_response()
}
