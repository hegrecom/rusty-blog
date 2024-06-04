use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::error_response::ErrorResponse;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Password is incorrect")]
    Unauthorized,
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Unauthorized => {
                (StatusCode::UNAUTHORIZED, ErrorResponse::from(self)).into_response()
            }
            Error::BadRequest(_) => {
                (StatusCode::BAD_REQUEST, ErrorResponse::from(self)).into_response()
            }
            Error::InternalServerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse::from(self)).into_response()
            }
        }
    }
}
