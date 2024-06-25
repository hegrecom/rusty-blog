use crate::post::post::Post;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Response {
    data: serde_json::Value,
    meta: serde_json::Value,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct PostsResponse {
    data: Vec<Post>,
    meta: PageMeta,
}

impl PostsResponse {
    pub fn data(&self) -> Vec<Post> {
        self.data.clone()
    }

    pub fn meta(&self) -> &PageMeta {
        &self.meta
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct PageMeta {
    page: i32,
    per: i32,
    total: i32,
    total_pages: i32,
}

impl TryFrom<Response> for PostsResponse {
    type Error = serde_json::Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        let data: Vec<Post> = serde_json::from_value(response.data)?;
        let meta = serde_json::from_value(response.meta)?;

        Ok(PostsResponse { data, meta })
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ErrorResponse {
    data: serde_json::Value,
    meta: ErrorMessage,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ErrorMessage {
    message: String,
}

impl TryFrom<Response> for ErrorResponse {
    type Error = serde_json::Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        let data = serde_json::from_value(response.data)?;
        let meta = serde_json::from_value(response.meta)?;

        Ok(ErrorResponse { data, meta })
    }
}
