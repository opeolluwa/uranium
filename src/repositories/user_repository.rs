use std::sync::Arc;

use sqlx::{Pool, Postgres, query_as};
use uuid::Uuid;

use crate::{
    entities::user::UserEntity,
    errors::{database_error::DatabaseError, service_error::ServiceError, user_service::UserServiceError},
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
}

impl UserRepositoryTrait for UserRepository {
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
                .map_err(|err| {
                    log::error!(
                        "error setting the account status due to {}",
                        err.to_string()
                    );
                    DatabaseError::from(err)
                })?;

        Ok(())
    }
}
