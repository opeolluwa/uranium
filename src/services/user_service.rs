use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::adapters::dto::user::UserDto;
use crate::adapters::requests::auth::CreateUserRequest;
use crate::errors::user_service_error::UserServiceError;
use crate::repositories::user_repository::{UserRepository, UserRepositoryTrait};

use super::user_helper_service::{UserHelperService, UserHelperServiceTrait};

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
    ) -> Result<(), UserServiceError>;

    async fn fetch_user_data(&self, user_identifier: Uuid) -> Result<UserDto, UserServiceError>;
}

impl UserServiceTrait for UserService {
    async fn create_user_account(
        &self,
        request: &CreateUserRequest,
    ) -> Result<(), UserServiceError> {
        if self
            .user_repository
            .find_by_email(&request.email)
            .await
            .is_some()
        {
            return Err(UserServiceError::ConflictError(
                "User already exists".to_owned(),
            ));
        }

        let password_hash = self.user_helper_service.hash_password(&request.password)?;
        let user = CreateUserRequest {
            password: password_hash,
            first_name: request.first_name.to_owned(),
            email: request.email.to_owned(),
            last_name: request.last_name.to_owned(),
        };

        self.user_repository.create_user(user).await
    }

    async fn fetch_user_data(&self, user_identifier: Uuid) -> Result<UserDto, UserServiceError> {
        todo!()
    }
}
