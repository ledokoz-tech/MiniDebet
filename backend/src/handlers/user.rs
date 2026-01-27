use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    Json as AxumJson,
};
use serde::{Deserialize, Serialize};
use crate::db::Db;
use crate::models::{User, NewUser};
use bcrypt::{hash, DEFAULT_COST};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub tax_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
}

pub async fn create_user(
    State(db): State<Db>,
    AxumJson(payload): AxumJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<CreateUserResponse>), (StatusCode, String)> {
    // Hash the password
    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password".to_string()))?;

    // Create new user
    let new_user = NewUser::new(
        payload.email.clone(),
        password_hash,
        payload.first_name.clone(),
        payload.last_name.clone(),
        payload.company_name.clone(),
        payload.tax_id.clone(),
    );

    // Save to database (placeholder)
    let user = User::new(
        new_user.email,
        "hashed_password_placeholder".to_string(), // In real implementation, use the hashed password
        new_user.first_name,
        new_user.last_name,
        new_user.company_name,
        new_user.tax_id,
    );

    let response = CreateUserResponse {
        id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        company_name: user.company_name,
    };

    Ok((StatusCode::CREATED, Json(response)))
}