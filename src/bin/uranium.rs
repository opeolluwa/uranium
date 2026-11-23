#![warn(unused_extern_crates)]

use std::net::{Ipv4Addr, SocketAddrV4};

use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::{HeaderName, HeaderValue, StatusCode},
};
use tower_http::{
    compression::CompressionLayer, limit::RequestBodyLimitLayer,
    sensitive_headers::SetSensitiveHeadersLayer, timeout::TimeoutLayer,
};
use uranium_lib::config::db::init_db_pool;
use uranium_lib::config::redis::init_redis_pool;
use uranium_lib::config::{
    cors::init_cors, env::load_config, logger::init_tracing, shutdown::shutdown_signal,
};
use uranium_lib::errors::app_error::AppError;
use uranium_lib::routes::router::load_routes as load_http_routes;

const SECURITY_HEADERS: &[(&str, &str)] = &[
    ("x-content-type-options", "nosniff"),
    ("x-frame-options", "DENY"),
    ("referrer-policy", "no-referrer"),
    (
        "permissions-policy",
        "geolocation=(), microphone=(), camera=()",
    ),
    ("cross-origin-opener-policy", "same-origin"),
    ("cross-origin-resource-policy", "same-origin"),
    (
        "content-security-policy",
        "default-src 'none'; frame-ancestors 'none'; base-uri 'none'; form-action 'none'",
    ),
];
#[tokio::main]
async fn main() -> Result<(), AppError> {
    let app_config = load_config()?;

    init_tracing(&app_config);

    let db_conn = init_db_pool(&app_config).await?;
    let redis_conn = init_redis_pool(&app_config).await?;
    let http_routes = load_http_routes(db_conn, &app_config, redis_conn)?;

    let app = Router::new()
        .merge(http_routes)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            app_config.body_limit_megabytes * 1024 * 1024,
        ))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            app_config.requests_time_out_secs,
        ))
        .layer(CompressionLayer::new())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(SetSensitiveHeadersLayer::new([
            axum::http::header::AUTHORIZATION,
            axum::http::header::COOKIE,
            axum::http::header::SET_COOKIE,
        ]))
        .layer(init_cors(&app_config))
        .layer({
            let is_production = app_config.is_production();
            axum::middleware::map_response(
                move |mut response: axum::response::Response| async move {
                    let headers = response.headers_mut();
                    for &(name, value) in SECURITY_HEADERS {
                        headers.insert(
                            HeaderName::from_static(name),
                            HeaderValue::from_static(value),
                        );
                    }
                    if is_production {
                        headers.insert(
                            HeaderName::from_static("strict-transport-security"),
                            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
                        );
                    }
                    response
                },
            )
        });

    let ip_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, app_config.port);

    tracing::info!("Service health check at http://{}/health", ip_address);
    let listener = tokio::net::TcpListener::bind(ip_address).await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    tracing::info!("Server shutdown completed");
    Ok(())
}
