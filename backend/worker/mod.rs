use worker::*;
use serde_json::json;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!("Received request: {:?}", req.url());
    
    // Handle CORS preflight requests
    if req.method() == Method::Options {
        return handle_cors_preflight();
    }
    
    let router = Router::new();
    
    router
        .get("/", |_, _| Response::ok("MiniDebet Worker API"))
        .get("/health", |_, _| Response::ok("OK"))
        .post_async("/api/auth/register", register_handler)
        .post_async("/api/auth/login", login_handler)
        .run(req, env)
        .await
}

fn handle_cors_preflight() -> Result<Response> {
    let cors_headers = Headers::new();
    cors_headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    cors_headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")?;
    cors_headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization")?;
    cors_headers.set("Access-Control-Max-Age", "86400")?;
    
    Response::empty()
        .unwrap()
        .with_status(204)
        .with_headers(cors_headers)
}

// Placeholder handlers - you'll need to implement these
async fn register_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let response_body = json!({
        "message": "Registration endpoint - implementation needed"
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
        "message": "Login endpoint - implementation needed"
    });
    
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "https://minidebet.pages.dev")?;
    headers.set("Content-Type", "application/json")?;
    
    Response::from_json(&response_body)
        .unwrap()
        .with_headers(headers)
}