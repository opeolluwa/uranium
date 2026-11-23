use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    adapters::{dto::user::UserDto, requests::auth::CreateUserRequest},
    entities::user::{self, Entity as Users},
    errors::{common_service_error::ServiceError, user_service_error::UserServiceError},
};

#[derive(Clone)]
pub struct UserRepository {
    db: DatabaseConnection,
}

impl UserRepository {
    pub fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

pub trait UserRepositoryTrait {
    fn find_by_identifier(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Option<user::Model>> + Send;

    fn find_by_email(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = Option<user::Model>> + Send;

    fn update_account_status(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn update_password(
        &self,
        identifier: &Uuid,
        new_password: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn create_user(
        &self,
        user: CreateUserRequest,
    ) -> impl std::future::Future<Output = Result<(), UserServiceError>> + Send;

    fn retrieve_information(
        &self,
        identifier: &Uuid,
    ) -> impl std::future::Future<Output = Result<UserDto, UserServiceError>> + Send;
}

impl UserRepositoryTrait for UserRepository {
    async fn create_user(&self, user: CreateUserRequest) -> Result<(), UserServiceError> {
        let model = user::ActiveModel {
            identifier: Set(Uuid::new_v4()),
            first_name: Set(user.first_name),
            last_name: Set(user.last_name),
            email: Set(user.email),
            password: Set(user.password),
            ..Default::default()
        };

        model
            .insert(&self.db)
            .await
            .map_err(|err| UserServiceError::OperationFailed(err.to_string()))?;

        Ok(())
    }

    async fn find_by_identifier(&self, identifier: &Uuid) -> Option<user::Model> {
        Users::find_by_id(*identifier).one(&self.db).await.ok()?
    }

    async fn find_by_email(&self, email: &str) -> Option<user::Model> {
        Users::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await
            .ok()?
    }

    async fn update_account_status(&self, identifier: &Uuid) -> Result<(), ServiceError> {
        let user = Users::find_by_id(*identifier)
            .one(&self.db)
            .await
            .map_err(|err| ServiceError::DatabaseError(err))?
            .ok_or_else(|| {
                ServiceError::DatabaseError(sea_orm::DbErr::RecordNotFound(
                    "user not found".to_string(),
                ))
            })?;

        let mut active: user::ActiveModel = user.into();
        active.is_active = Set(true);
        active
            .update(&self.db)
            .await
            .map_err(|err| ServiceError::DatabaseError(err))?;

        Ok(())
    }

    async fn update_password(
        &self,
        identifier: &Uuid,
        new_password: &str,
    ) -> Result<(), ServiceError> {
        let user = Users::find_by_id(*identifier)
            .one(&self.db)
            .await
            .map_err(|err| ServiceError::DatabaseError(err))?
            .ok_or_else(|| {
                ServiceError::DatabaseError(sea_orm::DbErr::RecordNotFound(
                    "user not found".to_string(),
                ))
            })?;

        let mut active: user::ActiveModel = user.into();
        active.password = Set(new_password.to_string());
        active
            .update(&self.db)
            .await
            .map_err(|err| ServiceError::DatabaseError(err))?;

        Ok(())
    }

    async fn retrieve_information(&self, identifier: &Uuid) -> Result<UserDto, UserServiceError> {
        let user = Users::find_by_id(*identifier)
            .one(&self.db)
            .await
            .map_err(|err| UserServiceError::OperationFailed(err.to_string()))?
            .ok_or_else(|| {
                UserServiceError::OperationFailed("user not found".to_string())
            })?;

        Ok(UserDto {
            identifier: user.identifier,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        })
    }
}
