use serde::Serialize;

use crate::core::into_json::IntoJson;

#[derive(Debug, Serialize)]
pub struct Token {
    token: String,
}

impl Token {
    pub fn new(token: String) -> Self {
        Token { token }
    }
}

impl IntoJson for Token {}
