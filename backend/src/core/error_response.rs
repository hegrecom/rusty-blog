use axum::{response::IntoResponse, Json};

use super::error::Error;

pub struct ErrorResponse {
    error: Error,
}

impl From<Error> for ErrorResponse {
    fn from(error: Error) -> Self {
        ErrorResponse { error }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        Json(serde_json::json!({
            "data": null,
            "meta": {
                "message": format!("{}", self.error.to_string()),
            }
        }))
        .into_response()
    }
}
