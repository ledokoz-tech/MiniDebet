use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub user_id: String,
    pub client_id: String,
    pub invoice_number: String,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub currency: String,
    pub subtotal: f64,
    pub tax_rate: f64,
    pub tax_amount: f64,
    pub total_amount: f64,
    pub status: String,
    pub notes: Option<String>,
    pub pdf_url: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub id: String,
    pub invoice_id: String,
    pub description: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub total_price: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewInvoice {
    pub user_id: String,
    pub client_id: String,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub currency: Option<String>,
    pub notes: Option<String>,
    pub items: Vec<NewInvoiceItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewInvoiceItem {
    pub description: String,
    pub quantity: i32,
    pub unit_price: f64,
}

impl Invoice {
    pub fn new(
        user_id: String,
        client_id: String,
        invoice_number: String,
        issue_date: NaiveDate,
        due_date: NaiveDate,
        currency: String,
        subtotal: f64,
        tax_rate: f64,
        tax_amount: f64,
        total_amount: f64,
        notes: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            client_id,
            invoice_number,
            issue_date,
            due_date,
            currency,
            subtotal,
            tax_rate,
            tax_amount,
            total_amount,
            status: "draft".to_string(),
            notes,
            pdf_url: None,
            sent_at: None,
            paid_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl InvoiceItem {
    pub fn new(
        invoice_id: String,
        description: String,
        quantity: i32,
        unit_price: f64,
        total_price: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            invoice_id,
            description,
            quantity,
            unit_price,
            total_price,
            created_at: Utc::now(),
        }
    }
}