use std::env;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use crate::{
    admin::{dto::token::Token, vo::claims::Claims},
    core::error::Error,
};

use super::token_service::TokenService;

pub struct JwtTokenService {
    secret: String,
    expires_in: i64,
}

impl JwtTokenService {
    const DEFAULT_EXPIRATION: i64 = 60 * 60 * 24;

    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let expires_in = JwtTokenService::DEFAULT_EXPIRATION;

        JwtTokenService { secret, expires_in }
    }
}

impl TokenService for JwtTokenService {
    fn generate_token(&self) -> Result<Token, Error> {
        let claims = Claims::new(self.expires_in);
        let token_string = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|err| Error::InternalServerError(err.to_string()))?;

        Ok(Token::new(token_string))
    }

    fn verify_token(&self, token: Token) -> Result<bool, Error> {
        jsonwebtoken::decode::<Claims>(
            token.value(),
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_err| Error::Unauthorized)?;

        Ok(true)
    }
}
