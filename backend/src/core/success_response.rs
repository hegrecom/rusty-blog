use axum::{http::StatusCode, response::IntoResponse, Json};

use super::into_json::IntoJson;

pub struct SuccessResponseBuilder {
    status_code: Option<StatusCode>,
    data: Option<serde_json::Value>,
    meta: Option<serde_json::Value>,
}

impl SuccessResponseBuilder {
    pub fn new() -> Self {
        SuccessResponseBuilder {
            status_code: None,
            data: None,
            meta: None,
        }
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = Some(status_code);
        self
    }

    pub fn data<T: IntoJson>(mut self, data: T) -> Self {
        self.data = Some(data.into_json());
        self
    }

    pub fn meta<T: IntoJson>(mut self, meta: T) -> Self {
        self.meta = Some(meta.into_json());
        self
    }

    pub fn build(self) -> SuccessResponse {
        SuccessResponse {
            status_code: self.status_code.unwrap_or(StatusCode::OK),
            data: self.data.unwrap_or(serde_json::json!(null)),
            meta: self.meta.unwrap_or(serde_json::json!(null)),
        }
    }
}

pub struct SuccessResponse {
    status_code: StatusCode,
    data: serde_json::Value,
    meta: serde_json::Value,
}

impl IntoResponse for SuccessResponse {
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code,
            Json(serde_json::json!({
                "data": self.data,
                "meta": self.meta,
            })),
        )
            .into_response()
    }
}
