use std::env;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub backend: Backend,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            backend: Backend {
                host: "http://localhost:8000".to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Backend {
    pub host: String,
}
