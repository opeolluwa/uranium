use axum::{extract::Extension, http::StatusCode, routing::get_service, Router};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr, path::PathBuf};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// the application route controllers/handlers
mod controllers;
/// the application model definitions, eg: User model, Todo Model e.t.c
mod models;
/// the application routing logic
mod routes;
///modules shared across the application, like API response patters, pagination logic e.t.c
mod shared;

#[tokio::main]
///the application entry point
async fn main() {
    //logger
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // parse the .env file in development
    dotenv().ok();
    //try parsing database connection string
    //TODO" add graceful shutdown
    let database_connection_string =
        env::var("DATABASE_URL").expect("database URL is not provided in env variable");

    //database connection pool
    let database = PgPoolOptions::new()
        .max_connections(5)
        // .connect_timeout(Duration::from_secs(4))
        .connect(&database_connection_string)
        .await
        .expect("Could not connect to database ");
    println!("Successfully connected to database");

    //static file mounting
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("views");
    let static_files_service = get_service(
        ServeDir::new(assets_dir).append_index_html_on_directories(true),
    )
    .handle_error(|error: std::io::Error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            //TODO" add graceful shutdown
            format!("Unhandled internal error: {}", error),
        )
    });

    //initialize cors layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    //mount the app routes and middleware
    let app = Router::new()
        .fallback(static_files_service)
        .nest("/api/v1/", routes::root::router())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(database));

    //mount the server to an ip address
    /*
     * if you can read the environment variable value for PORt from the .env
     * parse the read value into the variable value_from_env els use 8405
     */
    let port = env::var("PORT")
        .ok()
        .and_then(|value_from_env| value_from_env.parse().ok())
        .unwrap_or(4835);
    /*
     * if there is an env value,
     * try the parse the value to determine of the environment is development or production
     * else, assign the localhost ip address to catch error an fall through
     */

    let ip_address = match env::var("ENVIRONMENT") {
        /*
         * if the environment is production, use the derived port and the placeholder address
         * else use the default localhost IP address and a chosen port
         */
        Ok(env) => {
            if env == String::from("production").trim() {
                //return the placeholder address and the computed port
                SocketAddr::from(([0, 0, 0, 0], port))
            } else {
                //return localhost IP address
                SocketAddr::from(([127, 0, 0, 1], port))
            }
        }

        _ =>
        /*
         * return the localhost IP address as a fall through
         * if the address cannot be found, or badly constructed
         */
        {
            SocketAddr::from(([127, 0, 0, 1], port))
        }
    };
    println!("Ignition started on http://{}", &ip_address);
    /*  let otp = shared::otp_handler::generate_otp();
       let is_valid_otp = shared::otp_handler::validate_otp(&500256);
       println!("{otp}, is valid otp {is_valid_otp}");
    */
    //launch the server
    axum::Server::bind(&ip_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
