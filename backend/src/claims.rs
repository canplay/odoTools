use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const JWT_EXPIRATION_HOURS: i64 = 1;
const SECRET: &str = "CANPLAY";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub password: String,
    pub permissions: Vec<String>,
    exp: i64,
}

impl Claims {
    pub fn new(username: String, password: String, permissions: Vec<String>) -> Self {
        Self {
            username,
            password,
            permissions,
            exp: (Utc::now() + Duration::hours(JWT_EXPIRATION_HOURS)).timestamp(),
        }
    }
}

pub fn create_jwt(claims: Claims) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}

pub fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let decoding_key = DecodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}
