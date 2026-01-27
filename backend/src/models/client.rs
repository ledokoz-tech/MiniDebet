use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Client {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub email: Option<String>,
    pub company: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: String,
    pub vat_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewClient {
    pub user_id: String,
    pub name: String,
    pub email: Option<String>,
    pub company: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub vat_number: Option<String>,
}

impl NewClient {
    pub fn new(
        user_id: String,
        name: String,
        email: Option<String>,
        company: Option<String>,
        street: Option<String>,
        city: Option<String>,
        postal_code: Option<String>,
        country: Option<String>,
        vat_number: Option<String>,
    ) -> Self {
        Self {
            user_id,
            name,
            email,
            company,
            street,
            city,
            postal_code,
            country: country.unwrap_or_else(|| "DE".to_string()),
            vat_number,
        }
    }
}

impl Client {
    pub fn new(
        user_id: String,
        name: String,
        email: Option<String>,
        company: Option<String>,
        street: Option<String>,
        city: Option<String>,
        postal_code: Option<String>,
        country: String,
        vat_number: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            name,
            email,
            company,
            street,
            city,
            postal_code,
            country,
            vat_number,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}