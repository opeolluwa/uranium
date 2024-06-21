#[derive(Clone)]
pub struct AppState {
    pub database: sea_orm::DatabaseConnection,
}
