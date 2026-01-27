use worker::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    timestamp: String,
    version: String,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!("MiniDebet Worker received request: {:?}", req.url());
    
    let router = Router::new();
    
    router
        .get("/", |_, _| Response::ok("MiniDebet Worker API"))
        .get("/health", |_, _| {
            let response = HealthResponse {
                status: "healthy".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                version: "1.0.0".to_string(),
            };
            Response::from_json(&response)
        })
        .get_async("/api/users", get_users)
        .post_async("/api/users", create_user)
        .run(req, env)
        .await
}

async fn get_users(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    // Placeholder implementation
    Response::ok("[]")
}

async fn create_user(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    // Placeholder implementation
    let _body = req.text().await?;
    Response::ok("{\"id\": \"user-123\"}")
}