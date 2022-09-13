use axum::{
    routing::{get, get_service},
    Router,
     http::StatusCode,
};
use std::{net::SocketAddr, path::PathBuf};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use std::env;
//local modules
mod config;
mod controllers;
mod routes;
mod shared;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    //static file mounting
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("views");
    let static_files_service = get_service(
        ServeDir::new(assets_dir).append_index_html_on_directories(true),
    )
    .handle_error(|error: std::io::Error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error),
        )
    });

    //connect to database
    // config::database::mongodb().await;
    println!("Successfully connected to database");

    //initialize cors layer
    let cors = CorsLayer::new().allow_origin(Any);
    //mount the app routes
    let app = Router::new()
        .fallback(static_files_service)
        .nest("/v1/", routes::root::router())
        // .route("/", get(|| async { "nitride" }))
        .layer(cors);
    //mount the server to an ip address
    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8052);
    let ip_address = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Ignition started on http://{}", &ip_address);
    //launch the server
    axum::Server::bind(&ip_address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
