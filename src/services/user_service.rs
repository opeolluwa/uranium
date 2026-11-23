use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::adapters::user::UserDto;
use crate::errors::user_service_error::UserServiceError;
use crate::repositories::user_repository::{UserRepository, UserRepositoryTrait};

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn init(db: &DatabaseConnection) -> Self {
        Self {
            user_repository: UserRepository::init(db),
        }
    }
}

pub(crate) trait UserServiceTrait {
    async fn retrieve_information(
        &self,
        user_identifier: Uuid,
    ) -> Result<UserDto, UserServiceError>;
}

impl UserServiceTrait for UserService {
    async fn retrieve_information(
        &self,
        user_identifier: Uuid,
    ) -> Result<UserDto, UserServiceError> {
        self.user_repository
            .retrieve_information(&user_identifier)
            .await
    }
}
