use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Create a new Axum router
    let app = Router::new()
        .route("/", get(root));

    // Define the address to serve on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler for the root endpoint
async fn root() -> &'static str {
    "Hello, Rust Web Server!"
}
