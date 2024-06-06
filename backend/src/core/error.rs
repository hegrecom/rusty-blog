use axum::{
    extract::Request,
    http::{StatusCode, Uri},
    middleware::Next,
    response::{IntoResponse, Response},
};

use super::error_response::ErrorResponse;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Password is incorrect")]
    Unauthorized,
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("No route for {0}")]
    NotFound(String),
    #[error("Record not found")]
    RecordNotFound,
    #[error("Method Not Allowed")]
    MethodNotAllowed,
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
            Error::NotFound(_) => {
                (StatusCode::NOT_FOUND, ErrorResponse::from(self)).into_response()
            }
            Error::RecordNotFound => {
                (StatusCode::NOT_FOUND, ErrorResponse::from(self)).into_response()
            }
            Error::MethodNotAllowed => {
                (StatusCode::METHOD_NOT_ALLOWED, ErrorResponse::from(self)).into_response()
            }
            Error::InternalServerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse::from(self)).into_response()
            }
        }
    }
}

pub async fn not_found_handler(uri: Uri) -> Result<(), Error> {
    Err(Error::NotFound(uri.to_string()))
}

pub async fn method_not_allowed_handler(req: Request, next: Next) -> Result<Response, Error> {
    let response = next.run(req).await;

    match response.status() {
        StatusCode::METHOD_NOT_ALLOWED => Err(Error::MethodNotAllowed),
        _ => Ok(response),
    }
}
