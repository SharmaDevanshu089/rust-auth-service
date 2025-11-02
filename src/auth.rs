// yaha auth inset logic hoga
use axum::{
    RequestPartsExt, async_trait,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::headers::{Authorization, HeaderMapExt, authorization::Bearer};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::env;

// HANDLER SE YAHA LAA RAHA HU EASY NESS FOR PROGRAM
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// YAHA PE CLAIMS HOLD HONGE
#[derive(Debug)]
pub struct AuthUser {
    pub claims: Claims,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.typed_get::<Authorization<Bearer>>();

        let token = match auth_header {
            Some(Authorization(bearer)) => bearer.token().to_string(),
            None => {
                return Err((StatusCode::UNAUTHORIZED, "Missing token".to_string()));
            }
        };

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        );

        match token_data {
            Ok(token_data) => Ok(AuthUser {
                claims: token_data.claims,
            }),
            Err(e) => {
                let error_message = format!("Invalid token: {}", e);
                Err((StatusCode::UNAUTHORIZED, error_message))
            }
        }
    }
}
