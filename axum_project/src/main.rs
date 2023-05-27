mod handlers;

use handlers::*;
use axum::{
    Router,
    routing::post
    };
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let client = AppState::make_pool().await;

    let app = Router::new()
        .route("/submit-form", post(handle_form))
        .with_state(client);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("listening on {}", addr);
        
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

