use async_trait::async_trait;
use sqlx::{Pool, Postgres};
/// a placeholder for building SQL query abstractions
/// the util implements a number of common SQL transactions like, save, get by primary key, get all, get one, e.t.c
/// #Example
///
/// ``` rust
/// #[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
/// struct User {
/// id: Uuid,
/// age: u32,
/// name: String
/// }
///
/// impl SqlQueryBuilder for User {
/// fn save(db_connection){
/// let sql_query = "INSERT INTO user_information (id, age, name) VALUES ($1, $2, $3) RETURNING *";
///    let otp = sqlx::query_as::<_, Otp>(sql_query)
///     .bind(self.id)
///     .bind(self.age)
///    .bind(self.name)
///  .fetch_one(db_connection)
/// .await;
///  }
/// }
/// ```

#[async_trait]
pub trait Create {
    /// allow generic use of the query builder for multiple models
    type Entity;
    type Attributes;

    /// save a new record in the database
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

#[async_trait]
pub trait FindEntity {
    type Entity;
    type Attributes;
    async fn find(
        &self,
        fields: Self::Attributes,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error>;
}
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
#[async_trait]
pub trait UpdateEntity {
    type Entity;
    type Attributes;
    async fn update(
        fields: Self::Attributes,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error>;
}

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
