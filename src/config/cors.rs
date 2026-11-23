use axum::http::{HeaderName, HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

use crate::config::env::{AppConfig, Environment};

pub fn init_cors(config: &AppConfig) -> CorsLayer {
    let cors = CorsLayer::new().allow_methods([
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::OPTIONS,
        Method::PATCH,
    ]);

    let cors = match config.environment {
        Environment::Production => cors
            .allow_headers([
                HeaderName::from_static("content-type"),
                HeaderName::from_static("authorization"),
                HeaderName::from_static("accept"),
                HeaderName::from_static("origin"),
                HeaderName::from_static("x-requested-with"),
            ])
            .expose_headers([
                HeaderName::from_static("content-type"),
                HeaderName::from_static("x-request-id"),
            ]),
        _ => cors.allow_headers(Any).expose_headers(Any),
    };

    let has_wildcard = config
        .allowed_origins
        .iter()
        .any(|o| o == "*" || o == "Any");

    if config.environment.is_production() && has_wildcard {
        panic!(
            "CORS: wildcard origin is not allowed in production; \
             restricting to same-origin only"
        );
    }

    let origins = config
        .allowed_origins
        .iter()
        .filter_map(|origin| origin.parse::<HeaderValue>().ok())
        .collect::<Vec<_>>();

    if origins.is_empty() {
        panic!("CORS: no valid origins configured; denying all cross-origin requests");
    }

    match config.environment {
        Environment::Production => cors.allow_origin(origins),
        _ => cors.allow_origin(Any),
    }
}
