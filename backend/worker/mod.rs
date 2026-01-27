use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!("Received request: {:?}", req.url());
    
    let router = Router::new();
    
    router
        .get("/", |_, _| Response::ok("MiniDebet Worker API"))
        .get("/health", |_, _| Response::ok("OK"))
        .run(req, env)
        .await
}