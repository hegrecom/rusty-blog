use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
}

impl Claims {
    pub fn new(expires_in: i64) -> Self {
        let current_epoch_time = chrono::Utc::now().timestamp();
        Claims {
            exp: current_epoch_time + expires_in,
        }
    }
}
