use worker::*;
use serde_json::json;

mod db;
mod auth;
mod handlers;

use handlers::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!("=== DEBUG: NEW WORKER VERSION ACTIVE ===");
    console_log!("DEBUG: Received request: {:?}", req.url());
    console_log!("DEBUG: Request method: {:?}", req.method());
    
    // Handle CORS preflight requests for all routes
    if req.method() == Method::Options {
        console_log!("DEBUG: Processing OPTIONS preflight request");
        return handle_cors_preflight();
    }
    
    let router = Router::new();
    
    router
        .get("/", |_, _| Response::ok("MiniDebet Worker API - NEW VERSION WITH CORS"))
        .get("/health", health_check)
        .options("/api/auth/register", |_, _| {
            console_log!("DEBUG: Handling OPTIONS for /api/auth/register");
            handle_cors_preflight()
        })
        .options("/api/auth/login", |_, _| {
            console_log!("DEBUG: Handling OPTIONS for /api/auth/login");
            handle_cors_preflight()
        })
        .options("/api/clients", |_, _| {
            console_log!("DEBUG: Handling OPTIONS for /api/clients");
            handle_cors_preflight()
        })
        .options("/api/invoices", |_, _| {
            console_log!("DEBUG: Handling OPTIONS for /api/invoices");
            handle_cors_preflight()
        })
        .options("/*catchall", |_, _| {
            console_log!("DEBUG: Handling OPTIONS catchall");
            handle_cors_preflight()
        })
        .post_async("/api/auth/register", register_user)
        .post_async("/api/auth/login", login_user)
        .post_async("/api/clients", create_client)
        .get_async("/api/clients", get_clients)
        .post_async("/api/invoices", create_invoice)
        .get_async("/api/invoices", get_invoices)
        .run(req, env)
        .await
}

fn handle_cors_preflight() -> Result<Response> {
    console_log!("DEBUG: Inside handle_cors_preflight function");
    
    let mut cors_headers = Headers::new();
    cors_headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    cors_headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")?;
    cors_headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization")?;
    cors_headers.set("Access-Control-Max-Age", "86400")?;
    cors_headers.set("Access-Control-Allow-Credentials", "true")?;
    
    console_log!("DEBUG: CORS headers set successfully");
    
    Ok(Response::empty()
        .unwrap()
        .with_status(204)
        .with_headers(cors_headers))
}