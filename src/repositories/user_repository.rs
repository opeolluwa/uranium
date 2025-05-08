use std::sync::Arc;

use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{entities::user::UserEntity, errors::user_service::UserServiceError};

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
    async fn find_by_identifier(&self, identifier: &Uuid) -> Option<UserEntity> {
        todo!()
    }

    async fn find_by_email(&self, email: &String) -> Option<UserEntity> {
        todo!()
    }

    async fn save(&self, identifier: &Uuid) -> Result<(), UserServiceError> {
        todo!()
    }

    async fn update_account_status(
        &self,
        identifier: &Uuid,
        status: &String,
    ) -> Result<(), UserServiceError> {
        todo!()
    }
}
