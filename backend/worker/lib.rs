use worker::*;
use serde_json::json;

mod db;
mod auth;
mod handlers;

use handlers::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!("Received request: {:?}", req.url());
    
    // Handle CORS preflight requests for all routes
    if req.method() == Method::Options {
        return handle_cors_preflight();
    }
    
    let router = Router::new();
    
    router
        .get("/", |_, _| Response::ok("MiniDebet Worker API"))
        .get("/health", health_check)
        .options("/*catchall", |_, _| handle_cors_preflight())
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
    let mut cors_headers = Headers::new();
    cors_headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    cors_headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")?;
    cors_headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization")?;
    cors_headers.set("Access-Control-Max-Age", "86400")?;
    cors_headers.set("Access-Control-Allow-Credentials", "true")?;
    
    Ok(Response::empty()
        .unwrap()
        .with_status(204)
        .with_headers(cors_headers))
}