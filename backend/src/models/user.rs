use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub tax_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub tax_id: Option<String>,
}

impl NewUser {
    pub fn new(
        email: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
        company_name: Option<String>,
        tax_id: Option<String>,
    ) -> Self {
        Self {
            email,
            password,
            first_name,
            last_name,
            company_name,
            tax_id,
        }
    }
}

impl User {
    pub fn new(
        email: String,
        password_hash: String,
        first_name: Option<String>,
        last_name: Option<String>,
        company_name: Option<String>,
        tax_id: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            email,
            password_hash,
            first_name,
            last_name,
            company_name,
            tax_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}