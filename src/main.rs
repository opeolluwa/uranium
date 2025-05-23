use errors::app_error::AppError;
use routes::router::load_routes;
use shared::extract_env::extract_env;
use sqlx::postgres::PgPoolOptions;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod adapters;
mod config;
mod controllers;
mod entities;
mod errors;
mod middlewares;
mod repositories;
mod routes;
mod services;
mod shared;
mod states;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = extract_env::<String>("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;

    let app = load_routes(pool);
    let ip_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 3000));
    log::info!("Application listening on {}", ip_address);

    let listener = tokio::net::TcpListener::bind(ip_address)
        .await
        .map_err(|err| AppError::OperationFailed(err.to_string()))?;
    axum::serve(listener, app)
        .await
        .map_err(|err| AppError::OperationFailed(err.to_string()))?;

    Ok(())
}
