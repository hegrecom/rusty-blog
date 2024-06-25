#[derive(Debug, thiserror::Error, serde::Deserialize, serde::Serialize, Clone)]
#[error("Backend client error: {message}")]
pub struct BackendClientError {
    message: String,
}

impl BackendClientError {
    pub fn new(message: String) -> Self {
        BackendClientError { message }
    }
}
