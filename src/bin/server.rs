use axum::{Router, routing::get};

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/health", get(health));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .expect("Failed to bind server");

    axum::serve(listener, app).await.expect("Server failed");
}
