use sqlx::{postgres::PgPoolOptions, Pool};
use std::env;

pub async fn database_pool() -> Pool<sqlx::Postgres> {
    let database_connection_string =
        env::var("DATABASE_URL").expect("database URL is not provided in env variable");
    PgPoolOptions::new()
        .max_connections(5)
        // .connect_timeout(Duration::from_secs(4))
        .connect(&database_connection_string)
        .await
        .expect("Could not connect to database ")
}
