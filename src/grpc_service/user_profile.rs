use crate::{
    database_connection::DatabaseConnection,
    interceptors::shared::parse_user_id,
    proto::user_profile::{
        user_profile_server::UserProfile, ProfileRequest, ProfileResponse, ProfileUpdateRequest,
        ProfileUpdateResponse,
    },
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use tonic::async_trait;
use bookmark_database_codegen::entities::prelude::*;
use bookmark_database_codegen::entities::user_information::{self};

#[derive(Default)]
pub struct UserProfileImplementation {}

#[async_trait]

impl UserProfile for UserProfileImplementation {
    async fn get_profile(
        &self,
        request: tonic::Request<ProfileRequest>,
    ) -> std::result::Result<tonic::Response<ProfileResponse>, tonic::Status> {
        let (metadata, _, _) = request.into_parts();
        let db_connection = &DatabaseConnection::new().await;

        let user_id = parse_user_id(&metadata)?;

        let user_id = uuid::Uuid::parse_str(&user_id).unwrap();
        let Some(user_data) = UserInformation::find_by_id(user_id)
            .one(db_connection)
            .await
            .map_err(|err| {
                log::error!("Error fetching user_information{:#?}", err);
                tonic::Status::not_found("A user with the provided email does not exist")
            })?
        else {
            return Err(tonic::Status::not_found(
                "A user with the provided email does not exist",
            ));
        };

        let message = ProfileResponse {
            email: user_data.email,
            last_name: user_data.last_name,
            first_name: user_data.first_name,
            id: user_data.id.to_string(),
        };

        Ok(tonic::Response::new(message))
    }
    async fn update_profile(
        &self,
        request: tonic::Request<ProfileUpdateRequest>,
    ) -> std::result::Result<tonic::Response<ProfileUpdateResponse>, tonic::Status> {
        let (metadata, _, payload) = request.into_parts();
        let db_connection = &DatabaseConnection::new().await;

        let user_id = parse_user_id(&metadata)?;

        let user_id = uuid::Uuid::parse_str(&user_id).unwrap();
        let Some(user_data) = UserInformation::find_by_id(user_id)
            .one(db_connection)
            .await
            .map_err(|err| {
                log::error!("Error fetching user_information{:#?}", err);
                tonic::Status::not_found("A user with the provided email does not exist")
            })?
        else {
            return Err(tonic::Status::not_found(
                "A user with the provided email does not exist",
            ));
        };

        // update the fields in the update was sent or use the prev val
        let first_name = &user_data.first_name;
        let last_name = &user_data.last_name;
        let email = &user_data.email;

        let mut user_data: user_information::ActiveModel = user_data.to_owned().into();
        user_data.first_name = Set(payload.first_name.unwrap_or(first_name.to_string()));
        user_data.last_name = Set(payload.last_name.unwrap_or(last_name.to_string()));
        user_data.email = Set(payload.email.unwrap_or(email.to_string()));

        let updated_user_data: user_information::Model = user_data
            .update(db_connection)
            .await
            .map_err(|_| tonic::Status::internal("couldn't process request at this time "))?;

        let message = ProfileUpdateResponse {
            email: updated_user_data.email,
            last_name: updated_user_data.last_name,
            first_name: updated_user_data.first_name,
            id: updated_user_data.id.to_string(),
        };

        Ok(tonic::Response::new(message))
    }
}
