mod m20220101_000001_create_table;
mod m20241123_000002_add_two_factor_to_users;
mod m20241123_000003_create_otp_table;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241123_000002_add_two_factor_to_users::Migration),
            Box::new(m20241123_000003_create_otp_table::Migration),
        ]
    }
}
