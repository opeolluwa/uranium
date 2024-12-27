use std::fmt;

pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;


pub struct TableName(String);
impl sea_orm::Iden for TableName {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
        write!(s, "{}", self.0).unwrap();
    }
}

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration)
        ]
    }
}
