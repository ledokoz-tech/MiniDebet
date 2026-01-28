use worker::{Env, Result};
use serde::{Deserialize, Serialize};

// D1 Database wrapper for Cloudflare Workers
pub struct Database {
    env: Env,
}

impl Database {
    pub fn new(env: Env) -> Self {
        Self { env }
    }

    pub async fn get_d1(&self) -> Result<worker::d1::D1Database> {
        self.env.d1("DB")
    }

    // User operations
    pub async fn create_user(&self, user_data: &NewUser) -> Result<User> {
        let d1 = self.get_d1().await?;
        
        let query = "
            INSERT INTO users (id, email, password_hash, first_name, last_name, company_name, tax_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))
            RETURNING *
        ";
        
        let result = d1
            .prepare(query)
            .bind(&[
                serde_json::to_value(&user_data.id)?,
                serde_json::to_value(&user_data.email)?,
                serde_json::to_value(&user_data.password_hash)?,
                serde_json::to_value(&user_data.first_name)?,
                serde_json::to_value(&user_data.last_name)?,
                serde_json::to_value(&user_data.company_name)?,
                serde_json::to_value(&user_data.tax_id)?,
            ])
            .await?
            .first::<User>(None)
            .await?;

        result.ok_or_else(|| worker::Error::from("Failed to create user"))
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let d1 = self.get_d1().await?;
        
        let query = "SELECT * FROM users WHERE email = ?";
        let result = d1
            .prepare(query)
            .bind(&[serde_json::to_value(email)?])
            .await?
            .first::<User>(None)
            .await?;

        Ok(result)
    }

    // Client operations
    pub async fn create_client(&self, client_data: &NewClient) -> Result<Client> {
        let d1 = self.get_d1().await?;
        
        let query = "
            INSERT INTO clients (id, user_id, name, email, company, street, city, postal_code, country, vat_number, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))
            RETURNING *
        ";
        
        let result = d1
            .prepare(query)
            .bind(&[
                serde_json::to_value(&client_data.id)?,
                serde_json::to_value(&client_data.user_id)?,
                serde_json::to_value(&client_data.name)?,
                serde_json::to_value(&client_data.email)?,
                serde_json::to_value(&client_data.company)?,
                serde_json::to_value(&client_data.street)?,
                serde_json::to_value(&client_data.city)?,
                serde_json::to_value(&client_data.postal_code)?,
                serde_json::to_value(&client_data.country)?,
                serde_json::to_value(&client_data.vat_number)?,
            ])
            .await?
            .first::<Client>(None)
            .await?;

        result.ok_or_else(|| worker::Error::from("Failed to create client"))
    }

    pub async fn get_clients_by_user(&self, user_id: &str) -> Result<Vec<Client>> {
        let d1 = self.get_d1().await?;
        
        let query = "SELECT * FROM clients WHERE user_id = ? ORDER BY created_at DESC";
        let result = d1
            .prepare(query)
            .bind(&[serde_json::to_value(user_id)?])
            .await?
            .all()
            .await?;

        result.deserialize()
    }

    // Invoice operations
    pub async fn create_invoice(&self, invoice_data: &NewInvoice) -> Result<Invoice> {
        let d1 = self.get_d1().await?;
        
        let query = "
            INSERT INTO invoices (id, user_id, client_id, invoice_number, issue_date, due_date, currency, subtotal, tax_rate, tax_amount, total_amount, status, notes, pdf_url, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))
            RETURNING *
        ";
        
        let result = d1
            .prepare(query)
            .bind(&[
                serde_json::to_value(&invoice_data.id)?,
                serde_json::to_value(&invoice_data.user_id)?,
                serde_json::to_value(&invoice_data.client_id)?,
                serde_json::to_value(&invoice_data.invoice_number)?,
                serde_json::to_value(&invoice_data.issue_date)?,
                serde_json::to_value(&invoice_data.due_date)?,
                serde_json::to_value(&invoice_data.currency)?,
                serde_json::to_value(&invoice_data.subtotal)?,
                serde_json::to_value(&invoice_data.tax_rate)?,
                serde_json::to_value(&invoice_data.tax_amount)?,
                serde_json::to_value(&invoice_data.total_amount)?,
                serde_json::to_value(&invoice_data.status)?,
                serde_json::to_value(&invoice_data.notes)?,
                serde_json::to_value(&invoice_data.pdf_url)?,
            ])
            .await?
            .first::<Invoice>(None)
            .await?;

        result.ok_or_else(|| worker::Error::from("Failed to create invoice"))
    }

    pub async fn get_invoices_by_user(&self, user_id: &str) -> Result<Vec<Invoice>> {
        let d1 = self.get_d1().await?;
        
        let query = "SELECT * FROM invoices WHERE user_id = ? ORDER BY created_at DESC";
        let result = d1
            .prepare(query)
            .bind(&[serde_json::to_value(user_id)?])
            .await?
            .all()
            .await?;

        result.deserialize()
    }
}

// Data Models for D1
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub tax_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub tax_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewClient {
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub user_id: String,
    pub client_id: String,
    pub invoice_number: String,
    pub issue_date: String,
    pub due_date: String,
    pub currency: String,
    pub subtotal: f64,
    pub tax_rate: f64,
    pub tax_amount: f64,
    pub total_amount: f64,
    pub status: String,
    pub notes: Option<String>,
    pub pdf_url: Option<String>,
    pub sent_at: Option<String>,
    pub paid_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewInvoice {
    pub id: String,
    pub user_id: String,
    pub client_id: String,
    pub invoice_number: String,
    pub issue_date: String,
    pub due_date: String,
    pub currency: String,
    pub subtotal: f64,
    pub tax_rate: f64,
    pub tax_amount: f64,
    pub total_amount: f64,
    pub status: String,
    pub notes: Option<String>,
    pub pdf_url: Option<String>,
}