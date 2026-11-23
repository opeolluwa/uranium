use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum OneTimePasswords {
    Table,
    Id,
    Code,
    OwnerId,
    Purpose,
    Attempts,
    IsRevoked,
    ExpiresAt,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Identifier,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OneTimePasswords::Table)
                    .if_not_exists()
                    .col(uuid(OneTimePasswords::Id).primary_key())
                    .col(string(OneTimePasswords::Code))
                    .col(uuid(OneTimePasswords::OwnerId))
                    .col(string(OneTimePasswords::Purpose))
                    .col(integer(OneTimePasswords::Attempts).default(0))
                    .col(boolean(OneTimePasswords::IsRevoked).default(false))
                    .col(timestamp_with_time_zone(OneTimePasswords::ExpiresAt))
                    .col(
                        timestamp_with_time_zone(OneTimePasswords::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OneTimePasswords::Table, OneTimePasswords::OwnerId)
                            .to(Users::Table, Users::Identifier)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(OneTimePasswords::Table)
                    .name("idx_otp_owner_purpose")
                    .col(OneTimePasswords::OwnerId)
                    .col(OneTimePasswords::Purpose)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OneTimePasswords::Table).to_owned())
            .await
    }
}
