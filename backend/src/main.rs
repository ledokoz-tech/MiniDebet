use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

mod db;
mod models;
mod handlers;
mod auth;

use db::init_db;
use handlers::{create_user, create_client, get_clients, create_invoice, get_invoices, get_invoice};
use auth::middleware::auth_middleware;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize database
    let db = init_db().await.expect("Failed to initialize database");
    
    // Build our application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/api/users", post(create_user))
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/clients", post(create_client).get(get_clients))
        .route("/api/invoices", post(create_invoice).get(get_invoices))
        .route("/api/invoices/:id", get(get_invoice))
        .layer(axum::middleware::from_fn(auth_middleware))
        .with_state(db)
        .layer(CorsLayer::permissive());

    // Run our application
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to MiniDebet API!"
}

async fn health_check() -> &'static str {
    "OK"
}