use sea_orm_migration::{prelude::*, sea_orm::EnumIter, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create the types
        manager
            .create_type(
                Type::create()
                    .as_enum(AccountStatus::Table)
                    .values([
                        AccountStatus::Active,
                        AccountStatus::Deactivated,
                        AccountStatus::Inactive,
                        AccountStatus::Suspended,
                    ])
                    .to_owned(),
            )
            .await?;

        //create the gender type
        manager
            .create_type(
                Type::create()
                    .as_enum(Gender::Table)
                    .values([Gender::Male, Gender::Female, Gender::Unspecified])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserInformation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserInformation::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserInformation::FirstName).string())
                    .col(ColumnDef::new(UserInformation::LastName).string())
                    .col(ColumnDef::new(UserInformation::MiddleName).string())
                    .col(ColumnDef::new(UserInformation::Username).string())
                    .col(ColumnDef::new(UserInformation::Email).string())
                    .col(ColumnDef::new(UserInformation::Password).string())
                    .col(ColumnDef::new(UserInformation::AccountStatus).enumeration(
                        UserInformation::AccountStatus,
                        [
                            AccountStatus::Active,
                            AccountStatus::Deactivated,
                            AccountStatus::Inactive,
                            AccountStatus::Suspended,
                        ],
                    ))
                    .col(ColumnDef::new(UserInformation::CreatedAt).date_time())
                    .col(ColumnDef::new(UserInformation::UpdatedAt).date_time())
                    .col(ColumnDef::new(UserInformation::LastAvailableAt).date_time())
                    .col(ColumnDef::new(UserInformation::FullName).string())
                    .col(ColumnDef::new(UserInformation::OtpId).string())
                    .col(ColumnDef::new(UserInformation::PhoneNumber).string())
                    .col(ColumnDef::new(UserInformation::Gender).enumeration(
                        UserInformation::Gender,
                        [Gender::Female, Gender::Male, Gender::Unspecified],
                    ))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserInformation::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(AccountStatus::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Gender::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum UserInformation {
    Table,
    Id,
    FirstName,
    LastName,
    MiddleName,
    Username,
    Email,
    Password,
    AccountStatus,
    CreatedAt,
    UpdatedAt,
    LastAvailableAt,
    FullName,
    OtpId,
    PhoneNumber,
    Gender, //...
}

#[derive(Iden, EnumIter)]
pub enum AccountStatus {
    Table,
    #[iden = "active"]
    Active,
    #[iden = "deactivated"]
    Deactivated,
    #[iden = "inactive"]
    Inactive,
    #[iden = "suspended"]
    Suspended,
}

#[derive(Iden, EnumIter)]
pub enum Gender {
    Table,
    #[iden = "female"]
    Female,
    #[iden = "male"]
    Male,
    #[iden = "unspecified"]
    Unspecified,
}
