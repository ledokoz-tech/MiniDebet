use worker::*;
use serde_json::json;

mod db;
mod auth;
mod handlers;

use handlers::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!("=== DEBUG VERSION 2 ACTIVE ===");
    console_log!("DEBUG: URL: {:?}", req.url());
    console_log!("DEBUG: Method: {:?}", req.method());
    
    let url = req.url()?;
    let method = req.method();
    
    // Handle CORS preflight requests explicitly
    if method == Method::Options {
        console_log!("DEBUG: Explicit OPTIONS handling for path: {:?}", url.path());
        return handle_cors_preflight();
    }
    
    // Build router
    let router = Router::new();
    
    router
        .get("/", |_, _| Response::ok("MiniDebet Worker API - DEBUG VERSION 2"))
        .get("/health", health_check)
        .post_async("/api/auth/register", register_user)
        .post_async("/api/auth/login", login_user)
        .post_async("/api/clients", create_client)
        .get_async("/api/clients", get_clients)
        .post_async("/api/invoices", create_invoice)
        .get_async("/api/invoices", get_invoices)
        // OPTIONS handlers
        .options("/api/auth/register", |_, _| {
            console_log!("DEBUG: OPTIONS /api/auth/register matched");
            handle_cors_preflight()
        })
        .options("/api/auth/login", |_, _| {
            console_log!("DEBUG: OPTIONS /api/auth/login matched");
            handle_cors_preflight()
        })
        .options("/api/clients", |_, _| {
            console_log!("DEBUG: OPTIONS /api/clients matched");
            handle_cors_preflight()
        })
        .options("/api/invoices", |_, _| {
            console_log!("DEBUG: OPTIONS /api/invoices matched");
            handle_cors_preflight()
        })
        .options("/*catchall", |_, _| {
            console_log!("DEBUG: OPTIONS catchall matched");
            handle_cors_preflight()
        })
        .run(req, env)
        .await
}

fn handle_cors_preflight() -> Result<Response> {
    console_log!("DEBUG: Inside handle_cors_preflight function");
    
    let mut cors_headers = Headers::new();
    let result1 = cors_headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev");
    console_log!("DEBUG: Set Allow-Origin header result: {:?}", result1.is_ok());
    
    let result2 = cors_headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS");
    console_log!("DEBUG: Set Allow-Methods header result: {:?}", result2.is_ok());
    
    let result3 = cors_headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization");
    console_log!("DEBUG: Set Allow-Headers header result: {:?}", result3.is_ok());
    
    let result4 = cors_headers.set("Access-Control-Max-Age", "86400");
    console_log!("DEBUG: Set Max-Age header result: {:?}", result4.is_ok());
    
    let result5 = cors_headers.set("Access-Control-Allow-Credentials", "true");
    console_log!("DEBUG: Set Allow-Credentials header result: {:?}", result5.is_ok());
    
    console_log!("DEBUG: All CORS headers set, creating response");
    
    let response = Response::empty()
        .unwrap()
        .with_status(204)
        .with_headers(cors_headers);
    
    console_log!("DEBUG: CORS preflight response created successfully");
    Ok(response)
}