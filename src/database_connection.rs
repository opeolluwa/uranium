use std::time::Duration;
pub struct DatabaseConnection {}

impl DatabaseConnection {
    pub async fn new() -> sea_orm::DatabaseConnection {
        let mut opt =
            sea_orm::ConnectOptions::new(&crate::config::CONFIG.database_connection_string);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Debug);
        let db_connection = sea_orm::Database::connect(opt)
            .await
            .expect("Couldn't connect to database");

        db_connection
    }
}
