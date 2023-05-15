mod handlers;

use handlers::*;
use axum::{
    Router,
    routing::{get, post}
    };
use std::net::SocketAddr;
use tower_http::cors::{
    Any,
    CorsLayer
    };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let client = AppState::make_pool().await;

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/about/", get(about_handler))
        .route("/form/", get(form_handler))
        .route("/form/submit-form/", post(handle_form))
        .route("/form/success/", get(success_handler))
        .with_state(client)
        .layer(cors);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {}", addr);
        
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

