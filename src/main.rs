use axum::{routing::get, Router};
use dotenv;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
//local modules
mod config;
mod controllers;
mod routes;
mod shared;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    //try reading the environment variables
    dotenv::dotenv().expect("Failed to read .env file");
    //connect to database
    config::database::mongodb().await;
    println!("Successfully connected to database");

    //initialize cors layer
    let cors = CorsLayer::new().allow_origin(Any);
    //mount the app routes
    let app = Router::new()
        .nest("/v1/", routes::root::router())
        .route("/", get(|| async { "Nitrogen" }))
        .layer(cors);
    //mount the server to an ip address
    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8052);
    let ip_address = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Ignition started on http://{}", &ip_address);
    //launch the server
    axum::Server::bind(&ip_address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
