use axum::{Router, http::StatusCode, response::IntoResponse};
use sea_orm::DatabaseConnection;

use crate::{
    adapters::response::api_response::ApiResponseBuilder,
    config::env::AppConfig,
    errors::app_error::AppError,
    routes::{auth::authentication_routes, public::public_routes, users::user_routes},
    state::{Contracts, ServicesState},
};

pub fn load_routes(db: DatabaseConnection, app_config: &AppConfig) -> Result<Router, AppError> {
    let contracts = Contracts::new(app_config)?;
    let state = ServicesState::new(&db, &contracts);

    Ok(Router::new()
        .merge(public_routes(state.clone()))
        .merge(authentication_routes(state.clone()))
        .nest("/users", user_routes(state.clone()))
        .fallback(async || {
            ApiResponseBuilder::<()>::new()
                .message("the resource you're looking for does not exist or it has been permanently moved")
                .status_code(StatusCode::NOT_FOUND)
                .build()
                .into_response()
        }))
}
