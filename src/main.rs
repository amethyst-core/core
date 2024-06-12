#![forbid(unsafe_code)]

#![allow(unused_imports)]

mod db;
mod api;
mod server;

use crate::api::routes::router;
use crate::server::services::docker::DockerClient;

use crate::db::schema::create_schema;
use crate::db::queries::initialize_db;

use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::{http::header::HeaderValue, http::Method};
use tower_http::cors::CorsLayer;

/// This is the main function that starts the Amethyst Core server.
/// It sets up the CORS policy, defines the allowed methods and headers,
/// creates the router and starts the server on port 8000.
#[tokio::main]
async fn main() {

    let pool = initialize_db().await.expect("Failed to initialize database");
    create_schema(&pool).await.expect("Failed to create schema");
    let docker_client = DockerClient::new().expect("Failed to create Docker client");
    
    // Database Examples
    
    // Docker Examples
    
    // docker_client.pull_image().await.expect("Failed to pull image");
    
    // match docker_client.create_container("PAPER", "1.19.1", "25570", &pool).await {
    //         Ok(container_name) => {
    //     },
    //     Err(e) => eprintln!("Error creating container: {}", e),
    // }

    // match docker_client.list_containers().await {
    //     Ok(containers) => {
        //         for container in containers {
    //             println!("{:?}", container);
    //         }
    //     },
    //     Err(e) => eprintln!("Error: {}", e),
    // }

    // docker_client.delete_container("846b2a5db02520e0899d643a9d2209885124629ebdafde369d845ce7fcc63aa9", &pool).await.expect("Failed to start container");

    // Set up CORS policy
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    
    // Create the router
    let app = router().layer(cors);
    
    // Print startup message
    println!(
        "🔮 {}",
        "\x1b[35mAmethyst Core has successfully started\x1b[0m"
    );
    
    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
