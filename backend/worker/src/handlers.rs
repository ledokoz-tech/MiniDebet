use worker::{Request, Response, RouteContext, Result, Url};
use serde_json::json;
use uuid::Uuid;

use crate::db::{Database, NewUser, NewClient, NewInvoice};
use crate::auth::{AuthService, LoginRequest, User};

pub async fn health_check(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("OK")
}

pub async fn register_user(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = Database::new(ctx.env);
    
    // Parse request body
    let user_data: NewUser = req.json().await?;
    
    // Hash password
    let password_hash = AuthService::hash_password(&user_data.password_hash)?;
    
    // Create user in database
    let new_user = NewUser {
        password_hash,
        ..user_data
    };
    
    let user = db.create_user(&new_user).await?;
    
    // Create response
    let response_body = json!({
        "message": "User registered successfully",
        "user": {
            "id": user.id,
            "email": user.email,
            "first_name": user.first_name,
            "last_name": user.last_name,
            "company_name": user.company_name,
            "tax_id": user.tax_id
        }
    });
    
    let mut headers = worker::Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    headers.set("Content-Type", "application/json")?;
    
    Response::from_json(&response_body)
        .unwrap()
        .with_headers(headers)
}

pub async fn login_user(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = Database::new(ctx.env);
    
    // Parse request body
    let login_data: LoginRequest = req.json().await?;
    
    // Find user by email
    let user = db.find_user_by_email(&login_data.email).await?
        .ok_or_else(|| worker::Error::from("User not found"))?;
    
    // Verify password
    let is_valid = AuthService::verify_password(&login_data.password, &user.password_hash)?;
    
    if !is_valid {
        return Response::error("Invalid credentials", 401);
    }
    
    // Generate JWT token
    let token = AuthService::generate_token(&user.id, &user.email)?;
    
    // Create user response object
    let user_response = User {
        id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        company_name: user.company_name,
        tax_id: user.tax_id,
    };
    
    let login_response = AuthService::create_auth_response(user_response, token);
    
    let mut headers = worker::Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    headers.set("Content-Type", "application/json")?;
    
    Response::from_json(&login_response)
        .unwrap()
        .with_headers(headers)
}

pub async fn create_client(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Extract and verify JWT token
    let token = AuthService::extract_token_from_header(&req.headers())
        .ok_or_else(|| worker::Error::from("Missing authorization token"))?;
    
    let claims = AuthService::verify_token(&token)?;
    
    let db = Database::new(ctx.env);
    
    // Parse request body
    let mut client_data: NewClient = req.json().await?;
    client_data.user_id = claims.sub; // Set user_id from token
    
    // Create client in database
    let client = db.create_client(&client_data).await?;
    
    let response_body = json!({
        "message": "Client created successfully",
        "client": client
    });
    
    let mut headers = worker::Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    headers.set("Content-Type", "application/json")?;
    
    Response::from_json(&response_body)
        .unwrap()
        .with_headers(headers)
}

pub async fn get_clients(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Extract and verify JWT token
    let token = AuthService::extract_token_from_header(&req.headers())
        .ok_or_else(|| worker::Error::from("Missing authorization token"))?;
    
    let claims = AuthService::verify_token(&token)?;
    
    let db = Database::new(ctx.env);
    
    // Get clients for user
    let clients = db.get_clients_by_user(&claims.sub).await?;
    
    let response_body = json!({
        "clients": clients
    });
    
    let mut headers = worker::Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    headers.set("Content-Type", "application/json")?;
    
    Response::from_json(&response_body)
        .unwrap()
        .with_headers(headers)
}

pub async fn create_invoice(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Extract and verify JWT token
    let token = AuthService::extract_token_from_header(&req.headers())
        .ok_or_else(|| worker::Error::from("Missing authorization token"))?;
    
    let claims = AuthService::verify_token(&token)?;
    
    let db = Database::new(ctx.env);
    
    // Parse request body
    let mut invoice_data: NewInvoice = req.json().await?;
    invoice_data.user_id = claims.sub; // Set user_id from token
    
    // Create invoice in database
    let invoice = db.create_invoice(&invoice_data).await?;
    
    let response_body = json!({
        "message": "Invoice created successfully",
        "invoice": invoice
    });
    
    let mut headers = worker::Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    headers.set("Content-Type", "application/json")?;
    
    Response::from_json(&response_body)
        .unwrap()
        .with_headers(headers)
}

pub async fn get_invoices(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Extract and verify JWT token
    let token = AuthService::extract_token_from_header(&req.headers())
        .ok_or_else(|| worker::Error::from("Missing authorization token"))?;
    
    let claims = AuthService::verify_token(&token)?;
    
    let db = Database::new(ctx.env);
    
    // Get invoices for user
    let invoices = db.get_invoices_by_user(&claims.sub).await?;
    
    let response_body = json!({
        "invoices": invoices
    });
    
    let mut headers = worker::Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    headers.set("Content-Type", "application/json")?;
    
    Response::from_json(&response_body)
        .unwrap()
        .with_headers(headers)
}