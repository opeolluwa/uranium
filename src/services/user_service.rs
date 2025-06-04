use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::adapters::dto::user::UserDto;
use crate::adapters::requests::auth::CreateUserRequest;
use crate::errors::user_service::UserServiceError;
use crate::repositories::user_repository::UserRepository;

use super::user_helper_service::UserHelperService;

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository,
    user_helper_service: UserHelperService,
}

impl UserService {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            user_repository: UserRepository::init(pool),
            user_helper_service: UserHelperService::init(),
        }
    }
}

trait UserServiceTrait {
    async fn create_user_account(
        &self,
        request: &CreateUserRequest,
    ) -> Result<bool, UserServiceError>;

    async fn fetch_user_data(&self, user_identifier: Uuid) -> Result<UserDto, UserServiceError>;
}

impl UserServiceTrait for UserService {
    async fn create_user_account(
        &self,
        request: &CreateUserRequest,
    ) -> Result<bool, UserServiceError> {
        todo!()
    }

    async fn fetch_user_data(&self, user_identifier: Uuid) -> Result<UserDto, UserServiceError> {
        todo!()
    }
}
