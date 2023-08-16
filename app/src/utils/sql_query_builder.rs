
use async_trait::async_trait;
use serde_json::Value;
use sqlx::{Pool, Postgres};
/// Create - create a new database record
#[async_trait]
pub trait Create {
    type Entity;
    type Attributes;
    async fn create(
        fields: Self::Attributes,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error>;
}

/// delete model record
#[async_trait]
pub trait DeleteEntity {
    type Entity;
    type Attributes;
    async fn destroy(
        fields: Self::Attributes,
        db_connection: &Pool<Postgres>,
    ) -> Result<(), sqlx::Error>;
}

///find one base on the options provided
#[async_trait]
pub trait Find {
    type Entity;
    async fn find(
        fields: Value,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error>;
}

/// find user, create if not exist;
#[async_trait]
pub trait FindOrCreate {
    type Entity;
    type Attributes;
    async fn find_or_create(
        &self,
        fields: Self::Attributes,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error>;
}
#[async_trait]
pub trait FindAndCount {
    type Entity;
    type Attributes;
    async fn find_and_count(
        &self,
        fields: Self::Attributes,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error>;
}

/// update fields
#[async_trait]
pub trait UpdateEntity {
    type Entity;
    async fn update(
        &self,
        fields: Vec<std::collections::HashMap<String, String>>,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error>;
}

/// find a user by primary key
/// #Example
/// ```rust
///
/// #[async_trait]
/// impl FindByPk for UserModel {
/// type Entity = UserModel;
/// type Attributes = UserInformation;
///    async fn find_by_pk(
///       id: &str,
///       db_connection: &Pool<Postgres>,
///   ) -> Result<Self::Entity, sqlx::Error> {
///    sqlx::query_as::<_, UserModel>("SELECT * FROM user_information WHERE id = $1")
///     .bind(sqlx::types::Uuid::parse_str(id).unwrap())
///   .fetch_one(db_connection)
///      .await
/// }
///}
/// ```
#[async_trait]
pub trait FindByPk {
    type Entity;
    type Attributes;
    /// find record by id
    async fn find_by_pk(
        id: &str,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error>;
}
