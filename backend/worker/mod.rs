use worker::*;
use serde_json::json;

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
        .get("/health", |_, _| Response::ok("OK"))
        .options("/*catchall", |_, _| handle_cors_preflight())
        .post_async("/api/auth/register", register_handler)
        .post_async("/api/auth/login", login_handler)
        .post_async("/api/users", create_user_handler)
        .post_async("/api/clients", create_client_handler)
        .get_async("/api/clients", get_clients_handler)
        .post_async("/api/invoices", create_invoice_handler)
        .get_async("/api/invoices", get_invoices_handler)
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

// Auth handlers
async fn register_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let response_body = json!({
        "message": "Registration endpoint ready",
        "status": "success"
    });
    
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    headers.set("Content-Type", "application/json")?;
    
    Response::from_json(&response_body)
        .unwrap()
        .with_headers(headers)
}

async fn login_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let response_body = json!({
        "message": "Login endpoint ready",
        "status": "success"
    });
    
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    headers.set("Content-Type", "application/json")?;
    
    Response::from_json(&response_body)
        .unwrap()
        .with_headers(headers)
}

// Handler implementations
async fn create_user_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let response_body = json!({
        "message": "Create user endpoint ready",
        "status": "success"
    });
    
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    headers.set("Content-Type", "application/json")?;
    
    Response::from_json(&response_body)
        .unwrap()
        .with_headers(headers)
}

async fn create_client_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let response_body = json!({
        "message": "Create client endpoint ready",
        "status": "success"
    });
    
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    headers.set("Content-Type", "application/json")?;
    
    Response::from_json(&response_body)
        .unwrap()
        .with_headers(headers)
}

async fn get_clients_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let response_body = json!({
        "message": "Get clients endpoint ready",
        "status": "success",
        "data": []
    });
    
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    headers.set("Content-Type", "application/json")?;
    
    Response::from_json(&response_body)
        .unwrap()
        .with_headers(headers)
}

async fn create_invoice_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let response_body = json!({
        "message": "Create invoice endpoint ready",
        "status": "success"
    });
    
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    headers.set("Content-Type", "application/json")?;
    
    Response::from_json(&response_body)
        .unwrap()
        .with_headers(headers)
}

async fn get_invoices_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let response_body = json!({
        "message": "Get invoices endpoint ready",
        "status": "success",
        "data": []
    });
    
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    headers.set("Content-Type", "application/json")?;
    
    Response::from_json(&response_body)
        .unwrap()
        .with_headers(headers)
}