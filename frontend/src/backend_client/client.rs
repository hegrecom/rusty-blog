use std::env;

use chrono::TimeZone;
use leptos::logging;

use super::{error::BackendClientError, response::{PostsResponse, Response}};

pub struct BackendClient {
    base_url: String,
    client: reqwest::Client,
}

impl BackendClient {
    pub fn client() -> Self {
        BackendClient {
            base_url: env::var("BACKEND_HOST").expect("BACKEND_HOST must be set"),
            client: reqwest::ClientBuilder::new().build().unwrap(),
        }
    }

    pub async fn fetch_posts(&self, page: i32, per: i32) -> Result<PostsResponse, BackendClientError> {
        let response = self.client.get(&format!("{}/posts?page={}&per={}", self.base_url, page, per))
            .send()
            .await
            .map_err(|e| BackendClientError::new(e.to_string()))?
            .json::<Response>()
            .await
            .map_err(|e| BackendClientError::new(e.to_string()))?;

        response.try_into().map_err(|e: serde_json::Error| BackendClientError::new(e.to_string()))
    }
}
