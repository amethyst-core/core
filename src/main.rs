mod docker;
mod handlers;
mod routes;

use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::{http::header::HeaderValue, http::Method};
use routes::create_router;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().layer(cors);

    println!("🔮 Server has successfully started");
    let listner = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}
