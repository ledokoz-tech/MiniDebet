use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserSettings {
    pub user_id: String,
    pub default_tax_rate: f64,
    pub currency: String,
    pub invoice_prefix: String,
    pub next_invoice_number: i32,
    pub company_logo_url: Option<String>,
    pub payment_terms_days: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUserSettings {
    pub user_id: String,
    pub default_tax_rate: Option<f64>,
    pub currency: Option<String>,
    pub invoice_prefix: Option<String>,
    pub next_invoice_number: Option<i32>,
    pub company_logo_url: Option<String>,
    pub payment_terms_days: Option<i32>,
}

impl UserSettings {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            default_tax_rate: 19.0,
            currency: "EUR".to_string(),
            invoice_prefix: "INV".to_string(),
            next_invoice_number: 1,
            company_logo_url: None,
            payment_terms_days: 14,
            updated_at: Utc::now(),
        }
    }
}