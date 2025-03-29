use axum::{routing::get, Router};

async fn root_handler() -> &'static str {
    tracing::info!("Hello, Axum!");
    "Hello, Axum!"
}
async fn second_handler() -> &'static str {
    tracing::info!("Second handler");
    "Second handler"
}

#[tokio::main]
async fn main() {
    let bind_address = std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS is required");
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/second", get(second_handler));
    let listener = tokio::net::TcpListener::bind(bind_address.clone())
        .await
        .unwrap();
    tracing::info!("Up and running ... listening on {}", bind_address);
    axum::serve(listener, app).await.unwrap();
}
