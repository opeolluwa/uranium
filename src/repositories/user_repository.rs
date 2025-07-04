use std::sync::Arc;

use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    adapters::requests::auth::CreateUserRequest,
    entities::user::UserEntity,
    errors::{
        shared_service_error::ServiceError,
        user_service_error::UserServiceError,
    },
};

#[derive(Clone)]
pub struct UserRepository {
    pool: Arc<Pool<Postgres>>,
}

impl UserRepository {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}

pub trait UserRepositoryTrait {
    async fn find_by_identifier(&self, identifier: &Uuid) -> Option<UserEntity>;

    async fn find_by_email(&self, email: &str) -> Option<UserEntity>;

    async fn save(&self, identifier: &Uuid) -> Result<(), ServiceError>;

    async fn update_account_status(&self, identifier: &Uuid) -> Result<(), ServiceError>;

    async fn create_user(&self, user: CreateUserRequest) -> Result<(), UserServiceError>;
}

impl UserRepositoryTrait for UserRepository {
    async fn create_user(&self, user: CreateUserRequest) -> Result<(), UserServiceError> {
        let _ =
            sqlx::query(r#"INSERT INTO user (identifier, first_name, last_name,email,password"#)
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(user.first_name)
                .bind(user.last_name)
                .bind(user.email)
                .bind(user.password)
                .execute(self.pool.as_ref())
                .await
                .map_err(|err| UserServiceError::OperationFailed(err.to_string()))?;
        Ok(())
    }
    async fn find_by_identifier(&self, identifier: &Uuid) -> Option<UserEntity> {
        sqlx::query_as::<_, UserEntity>("SELECT * FROM users WHERE identifier = ?")
            .bind(identifier.to_string())
            .fetch_one(self.pool.as_ref())
            .await
            .ok()
    }

    async fn find_by_email(&self, email: &str) -> Option<UserEntity> {
        sqlx::query_as::<_, UserEntity>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_one(self.pool.as_ref())
            .await
            .ok()
    }

    async fn save(&self, identifier: &Uuid) -> Result<(), ServiceError> {
        todo!()
    }

    async fn update_account_status(&self, identifier: &Uuid) -> Result<(), ServiceError> {
        let _ =
            sqlx::query_as::<_, UserEntity>("UPDATE users SET is_active =?  WHERE identifier  = ?")
                .bind(true)
                .bind(identifier.to_string())
                .fetch_one(self.pool.as_ref())
                .await
                .map_err(|err| UserServiceError::OperationFailed(err.to_string()));

        Ok(())
    }
}
