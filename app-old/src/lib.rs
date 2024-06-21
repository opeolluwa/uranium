use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    RequestPartsExt, Router,
};

use crate::common::uranium::uranium_server::{Uranium, UraniumServer};
use crate::common::uranium::{HealthCheckRequest, HealthCheckResponse};
use tonic::{transport::Server, Request, Response as GrpcResponse, Status};

// use migration::{sea_orm::DatabaseConnection, Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use std::{collections::HashMap, env, net::SocketAddr, time::Duration};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::app_state::AppState;

mod common;
mod config;
mod extractors;
mod handlers;
mod router;
mod utils;

/// the grpc server
#[derive(Debug, Default)]
pub struct UraniumCore;

#[tonic::async_trait]
impl Uranium for UraniumCore {
    #[doc = " Returns the current health of the Uranium service."]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn health_check(
        &self,
        request: tonic::Request<HealthCheckRequest>,
    ) -> std::result::Result<tonic::Response<HealthCheckResponse>, tonic::Status> {
        let status = HealthCheckResponse {
            status: "Service Healthy\n".to_string(),
        };

        Ok(GrpcResponse::new(status))
    }
}

/// the grpc server
pub async fn grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let server = UraniumCore::default();

    Server::builder()
        .add_service(UraniumServer::new(server))
        .serve(addr)
        .await?;

    println!("Uranium gRPC server running on  {:?}", addr);
    Ok(())
}
// pub struct Uranium;
// impl Uranium {
/// Create a new instance of the application. This will load configuration and setup logging as well
pub async fn run() {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_connection_string =
        env::var("DATABASE_URL").expect("database URL is not provided in env variable");

    let mut opt = ConnectOptions::new(database_connection_string);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("uranium".to_owned());

    let connection = Database::connect(opt)
        .await
        .expect("error connecting to database ");

    //initialize cors layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let trace = TraceLayer::new_for_http();
    let state = AppState {
        database: connection,
    };
    // build our application with some routes
    let app = Router::new()
        .route("/", get(health_check))
        .nest("/:version/", router::routes(&state))
        .layer(trace)
        .layer(cors)
        .fallback(handle_404);

    // run the migration
    // Migrator::up(&connection, None).await.unwrap();
    /*    let port: u32 = std::env::var("PORT")
    .unwrap_or(53467.to_string())
    .parse::<u32>()
    .ok(); */

    let port = std::env::var("PORT")
        .ok()
        .expect("HTTP port not specified")
        .parse::<u16>()
        .unwrap_or(43467);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
// }

/// #health check
/// the health check is typically bounded to the server base URL to check the status of the app
/// this can b eeasliy done with cURl
/// for example
/// ```sh
/// curl 0.0.0.0:53467
/// ```
async fn health_check() -> &'static str {
    "Service is healthy\n"
}

/// 404 handler
async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        axum::response::Json(serde_json::json!({
        "success":false,
        "message":String::from("The requested resource does not exist on this server!"),
        })),
    )
}
#[derive(Debug)]
enum Version {
    V1,
    V2,
    V3,
}

#[async_trait]
impl<S> FromRequestParts<S> for Version
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let version = params
            .get("version")
            .ok_or_else(|| (StatusCode::NOT_FOUND, "version param missing").into_response())?;

        match version.as_str() {
            "v1" => Ok(Version::V1),
            "v2" => Ok(Version::V2),
            "v3" => Ok(Version::V3),
            _ => Err((StatusCode::NOT_FOUND, "unknown version").into_response()),
        }
    }
}
