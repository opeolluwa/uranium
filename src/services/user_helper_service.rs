use crate::errors::user_service::UserServiceError;
use crate::services::user_service::UserService;

#[derive(Clone)]
pub struct UserHelperService {}

impl UserHelperService {
    pub fn init() -> Self {
        Self {}
    }
}

pub trait UserHelperServiceTrait {
    async fn hash_password(&self, raw_password: &String) -> Result<String, UserServiceError>;

    async fn validate_password(
        &self,
        raw_password: &String,
        hash: &String,
    ) -> Result<bool, UserServiceError>;
}

impl UserHelperServiceTrait for UserService {
    async fn hash_password(&self, raw_password: &String) -> Result<String, UserServiceError> {
        todo!()
    }
    async fn validate_password(
        &self,
        raw_password: &String,
        hash: &String,
    ) -> Result<bool, UserServiceError> {
        todo!()
    }
}
