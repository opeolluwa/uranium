use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::entities::one_time_passwords::{self, Entity as OneTimePasswords};
use crate::errors::otp_service_error::OtpServiceError;

#[derive(Clone)]
pub struct OtpRepository {
    db: DatabaseConnection,
}

impl OtpRepository {
    pub fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn create_otp(
        &self,
        owner_id: Uuid,
        purpose: &str,
        hashed_code: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), OtpServiceError> {
        let model = one_time_passwords::ActiveModel {
            id: Set(Uuid::new_v4()),
            code: Set(hashed_code.to_string()),
            owner_id: Set(owner_id),
            purpose: Set(purpose.to_string()),
            expires_at: Set(expires_at.into()),
            ..Default::default()
        };

        model.insert(&self.db).await?;
        Ok(())
    }

    pub async fn find_active_otp(
        &self,
        owner_id: &Uuid,
        purpose: &str,
    ) -> Result<Option<one_time_passwords::Model>, OtpServiceError> {
        let otp = OneTimePasswords::find()
            .filter(one_time_passwords::Column::OwnerId.eq(*owner_id))
            .filter(one_time_passwords::Column::Purpose.eq(purpose))
            .filter(one_time_passwords::Column::IsRevoked.eq(false))
            .order_by_desc(one_time_passwords::Column::CreatedAt)
            .one(&self.db)
            .await?;

        Ok(otp)
    }

    pub async fn increment_attempts(
        &self,
        otp: one_time_passwords::Model,
    ) -> Result<i32, OtpServiceError> {
        let new_attempts = otp.attempts + 1;
        let mut active: one_time_passwords::ActiveModel = otp.into();
        active.attempts = Set(new_attempts);
        active.update(&self.db).await?;
        Ok(new_attempts)
    }

    pub async fn revoke_otp(&self, otp: one_time_passwords::Model) -> Result<(), OtpServiceError> {
        let mut active: one_time_passwords::ActiveModel = otp.into();
        active.is_revoked = Set(true);
        active.update(&self.db).await?;
        Ok(())
    }

    pub async fn revoke_all_for_owner(
        &self,
        owner_id: &Uuid,
        purpose: &str,
    ) -> Result<(), OtpServiceError> {
        one_time_passwords::Entity::update_many()
            .col_expr(
                one_time_passwords::Column::IsRevoked,
                sea_orm::sea_query::Expr::value(true),
            )
            .filter(one_time_passwords::Column::OwnerId.eq(*owner_id))
            .filter(one_time_passwords::Column::Purpose.eq(purpose))
            .filter(one_time_passwords::Column::IsRevoked.eq(false))
            .exec(&self.db)
            .await?;

        Ok(())
    }
}
