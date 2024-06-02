use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    password: String,
}

impl LoginRequest {
    pub fn password(&self) -> &str {
        &self.password
    }
}
