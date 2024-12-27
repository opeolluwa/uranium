use std::collections::HashMap;

use crate::adapters::email_templates::EmailBuilder;
use crate::adapters::email_templates::EmailTemplates;
use crate::adapters::kafka::Kafka;
use crate::database_connection::DatabaseConnection;
use crate::jwt::JwtClaims;
use crate::proto::authentication::authentication_server::Authentication;
use crate::proto::authentication::LoginRequest;
use crate::proto::authentication::LoginResponse;
use crate::proto::authentication::SignUpRequest;
use crate::proto::authentication::SignUpResponse;
use crate::proto::authentication::Status as RequestStatus;

use bcrypt::{verify, DEFAULT_COST};
use bookmark_database_codegen::entities::bookmark_collection;
use bookmark_database_codegen::entities::prelude::*;
use bookmark_database_codegen::entities::user_information::{self};

use kafka::producer::Record;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use tonic::Response;
use uuid::Uuid;

#[derive(Default)]
pub struct AuthenticationImplementation {}

#[tonic::async_trait]
impl Authentication for AuthenticationImplementation {
    async fn sign_up(
        &self,
        request: tonic::Request<SignUpRequest>,
    ) -> std::result::Result<tonic::Response<SignUpResponse>, tonic::Status> {
        let db_connection = &DatabaseConnection::new().await;
        let payload = request.into_inner();

        if UserInformation::find()
            .filter(user_information::Column::Email.eq(&payload.email))
            .one(db_connection)
            .await
            .expect("duplicate record")
            .is_some()
        {
            return Err(tonic::Status::already_exists(
                "A user with the provided email already exist",
            ));
        }
        let password = bcrypt::hash(payload.password, DEFAULT_COST).map_err(|_| {
            tonic::Status::unknown("The server couldn't process the request at this time")
        })?;

        let user_id = Uuid::new_v4();
        let default_vault = bookmark_collection::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set("default".into()),
            description: Set("default collection".into()),
            user_id: Set(user_id),
            ..Default::default()
        };

        let new_user = user_information::ActiveModel {
            id: Set(user_id),
            password: Set(password),
            first_name: Set(payload.first_name.trim().to_string().to_lowercase()),
            last_name: Set(payload.last_name.trim().to_string().to_lowercase()),
            email: Set(payload.email.trim().to_string().to_lowercase()),
            ..Default::default()
        };

        let _ = user_information::Entity::insert(new_user)
            .exec(db_connection)
            .await
            .map_err(|err| {
                tonic::Status::unknown(format!(
                    "The server couldn't process the request at this time sue to err {}",
                    err.to_string()
                ))
            })?;

        let _ = bookmark_collection::Entity::insert(default_vault)
            .exec(db_connection)
            .await
            .map_err(|err| {
                tonic::Status::unknown(format!(
                    "The server couldn't process the request at this time due to err {}",
                    err.to_string()
                ))
            })?;

        // send email

        let email_payload = EmailBuilder::new("Confirm email")
            .use_template(EmailTemplates::Signup)
            .send_to(&payload.email)
            .with_payload(HashMap::from([
                ("key".to_string(), "value".to_string()),
                ("more keys".to_string(), "more values".to_string()),
            ]))
            .to_bytes();

        let _ = Kafka::producer().send(&Record::from_value("bookmark", email_payload));
        Ok(Response::new(SignUpResponse {
            message: "Account Successfully Created".into(),
            status: RequestStatus::Ok.into(),
        }))
    }
    async fn login(
        &self,
        request: tonic::Request<LoginRequest>,
    ) -> std::result::Result<tonic::Response<LoginResponse>, tonic::Status> {
        let payload = request.into_inner();
        let db_connection = &DatabaseConnection::new().await;

        let Some(user_data) = UserInformation::find()
            .filter(user_information::Column::Email.eq(&payload.email))
            .one(db_connection)
            .await
            .map_err(|_| tonic::Status::not_found("Invalid email or password"))?
        else {
            return Err(tonic::Status::not_found("Invalid email or password"));
        };

        let is_correct_password = verify(payload.password, &user_data.password).map_err(|_| {
            return tonic::Status::not_found("Invalid email or password");
        })?;
        if !is_correct_password {
            return Err(tonic::Status::invalid_argument("Invalid email or password"));
        }

        // sign the token
        let Ok(jwt_token) =
            JwtClaims::new(user_data.email.clone(), user_data.id.clone().to_string()).gen_token()
        else {
            return Err(tonic::Status::internal(
                "error generating authorization header",
            ));
        };

        Ok(Response::new(LoginResponse {
            token: jwt_token,
            message: "User successfully logged in".into(),
        }))
    }

    // async fn forgotten_password(
    //     &self,
    //     request: tonic::Request<ForgottenPasswordRequest>,
    // ) -> std::result::Result<tonic::Response<ForgottenPasswordResponse>, tonic::Status> {
    //     let _payload = request.into_inner();

    //     //TODO: validate email
    //     //todo: Forward the message to the notification service
    //     let message = ForgottenPasswordResponse {
    //         status: 0,
    //         message: "".to_string(),
    //     };

    //     Ok(Response::new(message))
    // }
    // async fn set_new_password(
    //     &self,
    //     _request: tonic::Request<SetNewPasswordRequest>,
    // ) -> std::result::Result<tonic::Response<SetNewPasswordResponse>, tonic::Status> {
    //     todo!()
    // }
}
