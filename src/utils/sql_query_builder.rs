use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use std::future::Future;
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
pub trait SqlQueryBuilder {
    /// allow generic use of the query builder for multiple models
    type Entity;
    type Attributes;

    /// save a new record in the database
    async fn create(
        fields: Self::Attributes,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error>;
    /// find record by id
    async fn find_by_pk(
        id: &str,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error>;

    /* /// update model record
    async fn update(
        fields: Self::Attributes,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error>;

    /// update a field e.g user password
    async fn update_field(
        field: &str,
        value: &str,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error>;
    /// find model record
    async fn find(
        &self,
        fields: Self::Attributes,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error>;

    /// delete model record
    async fn destroy(
        fields: Self::Attributes,
        db_connection: &Pool<Postgres>,
    ) -> Result<(), sqlx::Error>; */
}
#[async_trait]
pub trait MyTrait {
    type Output: Future<Output = Result<(), String>>;
    async fn create(&self) -> Self::Output;
    async fn find(&self) -> Self::Output;
}

struct MyStruct;

#[async_trait]
impl MyTrait for MyStruct {
    type Output = Result<(), String>;
    async fn create(&self) -> Self::Output {
        Ok(())
    }
}
