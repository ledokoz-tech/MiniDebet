use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    Json as AxumJson,
};
use serde::{Deserialize, Serialize};
use crate::db::Db;
use crate::models::user::User;
use crate::auth::jwt::generate_token;
use bcrypt::{verify, hash, DEFAULT_COST};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(
    State(_db): State<Db>,
    AxumJson(payload): AxumJson<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    // In a real implementation, you'd query the database
    // This is a simplified example
    
    // For demo purposes, create a mock user
    let user = User::new(
        payload.email.clone(),
        hash(&payload.password, DEFAULT_COST).unwrap(),
        Some("Max".to_string()),
        Some("Mustermann".to_string()),
        Some("My Company GmbH".to_string()),
        Some("DE123456789".to_string()),
    );

    // Verify password (in real implementation, query from DB first)
    let is_valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    if !is_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // Generate JWT token
    let token = generate_token(&user)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate token".to_string()))?;

    let response = LoginResponse {
        token,
    };

    Ok(Json(response))
}