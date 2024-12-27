pub mod adapters;
pub mod config;
pub mod database_connection;
pub mod grpc_service;
pub mod interceptors;
pub mod jwt;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use self::config::CONFIG;
use bookmark_database_codegen::migration::{Migrator, MigratorTrait};
use grpc_service::authentication::AuthenticationImplementation;

use grpc_service::health_check::HealthCheckImplementation;
use grpc_service::user_profile::UserProfileImplementation;
use interceptors::authentication::check_and_validate_jwt;

use proto::authentication::authentication_server::AuthenticationServer;
use proto::health_check::health_check_server::HealthCheckServer;
use proto::user_profile::user_profile_server::UserProfileServer;
use tonic::service::interceptor;
use tonic::transport::Server;
use tonic::Request;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use uranium_grpc_codegen::server_stub as proto;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Error setting global tracing subscriber");

    let connection = sea_orm::Database::connect(&CONFIG.database_connection_string).await?;
    Migrator::up(&connection, None).await?;

    let address = SocketAddr::new(IpAddr::from(Ipv4Addr::UNSPECIFIED), CONFIG.port);

    let authentication_service = AuthenticationServer::new(AuthenticationImplementation::default());
    let health_check_service = HealthCheckServer::new(HealthCheckImplementation::default());

    let user_profile = UserProfileImplementation::default();
    let user_profile_service =
        UserProfileServer::with_interceptor(user_profile, check_and_validate_jwt);

    tracing::info!(message = "Starting server.", %address);
    Server::builder()
        .layer(interceptor(|request: Request<()>| {
            tracing::info!(">>>> incoming request {:#?}", request);
            Ok(request)
        }))
        .trace_fn(|_| tracing::info_span!("{:?}"))
        .timeout(Duration::from_secs(5))
        .add_service(authentication_service)
        .add_service(health_check_service)
        .add_service(user_profile_service)
        .serve(address)
        .await?;

    Ok(())
}
