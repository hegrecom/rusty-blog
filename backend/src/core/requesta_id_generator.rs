use axum::http::HeaderValue;
use tower_http::request_id::{MakeRequestId, RequestId};

#[derive(Clone, Default)]
pub struct RequestIdGenerator {}

impl RequestIdGenerator {
    fn generate_id() -> HeaderValue {
        let uuid = uuid::Uuid::new_v4();
        HeaderValue::from_str(&uuid.to_string()).unwrap()
    }
}

impl MakeRequestId for RequestIdGenerator {
    fn make_request_id<B>(&mut self, _request: &axum::http::Request<B>) -> Option<RequestId> {
        Some(RequestId::new(Self::generate_id()))
    }
}
