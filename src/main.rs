mod detector;
mod errors;
mod models;
mod uniswap;
mod validation;

use axum::{Json, Router, extract::State, routing::get};
use models::MonitoringResult;
use std::sync::Arc;

async fn health() -> &'static str {
    "TraceGuard is running!"
}

async fn alerts(State(result): State<Arc<MonitoringResult>>) -> Json<MonitoringResult> {
    Json((*result).clone())
}

#[tokio::main]
async fn main() {
    println!("TraceGuard is starting!");

    let result = match uniswap::fetch_recent_swaps().await {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Failed to fetch swaps: {error}");
            return;
        }
    };

    println!("Starting TraceGuard API...");

    let state = Arc::new(result);

    let app = Router::new()
        .route("/health", get(health))
        .route("/alerts", get(alerts))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind address");

    println!("TraceGuard API running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("API server failed");
}
