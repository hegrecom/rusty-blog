use axum::{async_trait, extract::FromRequestParts, http::header};
use regex::Regex;

use crate::admin::{
    dto::token::Token,
    service::{jwt_token_service::JwtTokenService, token_service::TokenService},
};

use super::error::Error;

pub struct RequireAuthorization;

#[async_trait]
impl<S> FromRequestParts<S> for RequireAuthorization
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Error> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());

        match auth_header {
            Some(auth_header) => {
                let token = Self::parse_authorization_header(auth_header)?;
                if Self::validate_token(token) {
                    Ok(Self)
                } else {
                    Err(Error::Unauthorized)
                }
            }
            _ => Err(Error::Unauthorized),
        }
    }
}

impl RequireAuthorization {
    fn parse_authorization_header(auth_header: &str) -> Result<&str, Error> {
        let re = Regex::new(r"^Bearer\s+(?P<token>\S+)$").unwrap();
        let token = re
            .captures(auth_header)
            .map_or(Err(Error::Unauthorized), |caps| {
                caps.name("token")
                    .map(|token| token.as_str())
                    .ok_or(Error::Unauthorized)
            });

        token
    }

    pub fn validate_token(token: &str) -> bool {
        let token = Token::new(token.to_string());
        JwtTokenService::new().verify_token(token).unwrap_or(false)
    }
}
