use axum::{http::StatusCode, response::IntoResponse, response::Json};

use serde_json::{json, Value};

use crate::{
    extractors::auth::CreateUser,
    utils::api_response::{ErrorResponse, SuccessResponse},
};

pub struct UserAuthenticationHandler;

impl UserAuthenticationHandler {
    pub async fn sign_up(
        Json(payload): Json<CreateUser>,
        // state: State<AppState>,
    ) -> Result<(StatusCode, Json<SuccessResponse<Value>>), ErrorResponse> {
        println!("{:?}", payload);
        // see if the user exists
        /*   let _user = UserInformation::find()
        .select_only()
        .columns([
            user_information::Column::Email,
            user_information::Column::Username,
        ])
        .one(&state.database)
        .await
        .unwrap(); */

        //build the response
        let response = SuccessResponse::new(
            "user successfully created",
            json!({ "userInformation": &payload }),
        );

        println!(" it got hete");
        //return the response
        Ok((StatusCode::CREATED, Json(response.await)))
    }

    /// verify with magic link
    pub async fn verify_magic_link() -> impl IntoResponse {}

    /// login user
    pub async fn login() -> impl IntoResponse {
        todo!()
    }

    /// logout user
    pub async fn logout() -> impl IntoResponse {
        todo!()
    }
}
