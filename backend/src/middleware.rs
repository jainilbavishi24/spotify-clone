use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,
}

// Simple JWT validation function for use in handlers
pub fn validate_jwt(token: &str) -> Result<Uuid, String> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            Uuid::parse_str(&token_data.claims.sub)
                .map_err(|_| "Invalid user ID in token".to_string())
        }
        Err(_) => Err("Invalid token".to_string()),
    }
}
