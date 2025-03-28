use axum::{
    routing::get,
    Router,
};

// Handler for the root endpoint (`/`)
async fn root_handler() -> &'static str {
    "Hello, Axum!"
}

#[tokio::main]
async fn main() {
    let bind_address = std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS is required");
    let app = Router::new()
        .route("/", get(root_handler));
//        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(bind_address.clone())
        .await
        .unwrap();
    tracing::info!("Up and running ... listening on {}", bind_address);
    axum::serve(listener, app).await.unwrap();
}