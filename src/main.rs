use anyhow::Context;
use axum::{http::StatusCode, routing::get_service, Router};
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr, path::PathBuf};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

//local modules
// mod config;
mod controllers;
mod routes;
mod shared;
mod models;

#[tokio::main]
async fn main() {
    //try parsing database connection string
    let database_connection_string = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| String::from("postgres://opeolluwa:thunderstorm@localhost/nitride"));

    //database connection pool
    let database = PgPoolOptions::new()
        .max_connections(5)
        // .connect_timeout(Duration::from_secs(4))
        .connect(&database_connection_string)
        .await
        .expect("Could not connect to database ");
    println!("Successfully connected to database");

    // This embeds database migrations in the application binary so we can ensure the database
    // is migrated correctly on startup
    // sqlx::migrate!().run(&database).await.unwrap();

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

    //initialize cors layer
    let cors = CorsLayer::new().allow_origin(Any);
    //mount the app routes
    let app = Router::new()
        .fallback(static_files_service)
        .nest("/api/v1/", routes::root::router())
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


}
