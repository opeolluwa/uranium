use axum::{http::StatusCode, response::IntoResponse, Json, Router};
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;

use crate::{
    adapters::message::Message,
    config::env::AppConfig,
    errors::app_error::AppError,
    routes::{auth::authentication_routes, public::public_routes, users::user_routes},
    state::{Contracts, ServicesState},
};

pub fn load_routes(
    db: DatabaseConnection,
    app_config: &AppConfig,
    redis: ConnectionManager,
) -> Result<Router, AppError> {
    let contracts = Contracts::new(app_config)?;
    let state = ServicesState::new(&db, &contracts, redis, app_config);

    Ok(Router::new()
        .merge(public_routes(state.clone()))
        .merge(authentication_routes(state.clone(), app_config))
        .nest("/users", user_routes(state.clone()))
        .fallback(async || {
            (
                StatusCode::NOT_FOUND,
                Json(Message::new("the resource you're looking for does not exist or it has been permanently moved")),
            )
                .into_response()
        }))
}
