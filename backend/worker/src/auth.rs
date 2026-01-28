use worker::{Request, Result, Headers};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use bcrypt::{hash, verify, DEFAULT_COST};
use std::time::{SystemTime, UNIX_EPOCH};

// JWT Secret - in production, use Workers Secrets
const JWT_SECRET: &str = "minidebet-jwt-secret-key-change-in-production";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user ID
    pub email: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub tax_id: Option<String>,
}

pub struct AuthService;

impl AuthService {
    pub fn generate_token(user_id: &str, email: &str) -> Result<String> {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize + 24 * 60 * 60; // 24 hours

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(JWT_SECRET.as_ref()),
        )
        .map_err(|e| worker::Error::from(format!("JWT encoding failed: {}", e)))
    }

    pub fn verify_token(token: &str) -> Result<Claims> {
        let validation = Validation::new(Algorithm::HS256);
        
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET.as_ref()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|e| worker::Error::from(format!("JWT verification failed: {}", e)))
    }

    pub fn hash_password(password: &str) -> Result<String> {
        hash(password, DEFAULT_COST)
            .map_err(|e| worker::Error::from(format!("Password hashing failed: {}", e)))
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        verify(password, hash)
            .map_err(|e| worker::Error::from(format!("Password verification failed: {}", e)))
    }

    pub fn extract_token_from_header(headers: &Headers) -> Option<String> {
        headers
            .get("Authorization")
            .ok()
            .flatten()
            .and_then(|auth_header| {
                if auth_header.starts_with("Bearer ") {
                    Some(auth_header[7..].to_string())
                } else {
                    None
                }
            })
    }

    pub fn create_auth_response(user: User, token: String) -> LoginResponse {
        LoginResponse { token, user }
    }
}