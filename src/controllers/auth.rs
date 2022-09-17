use crate::models::users::UserInformation;
// use crate::shared::api_response::{Error as ErrorResponse, Success as SuccessResponse};
use axum::Extension;
use axum::{/* http::StatusCode, */ response::IntoResponse, Json};
// use futures::FutureExt;
// use bcrypt::{hash, verify};
// use jsonwebtoken::{encode, EncodingKey, Header};
// use serde_json::json;
// use sqlx::postgres::PgRow;
use sqlx::PgPool;
// use std::env;
// use validator::Validate;

///create a new user
pub async fn sign_up(Json(_payload): Json<UserInformation>) -> impl IntoResponse {
    //destructure the request
    // todo!();
    print!("hey");
    //create new user
    /*  collection.insert_one(&user, None).await.unwrap();
    (
        StatusCode::CREATED,
        Json(json!({
            "success":true,
            "message":"user successfully created".to_string(),
            "data":None::<User>
        })),
    ) */
}

///login a new user
/// to login a user, fetch the request body and the database pool
/// use the pool to query the database for the user details in the request body
/// return result or error
/// Result<(StatusCode, Json<SuccessResponse<UserInformation>>), ErrorResponse<String>>
pub async fn login(
    Json(payload): Json<UserInformation>,
    Extension(database): Extension<PgPool>,
) -> impl IntoResponse {
    //destructure the payload to fetch user details
    let UserInformation {
        email, password, ..
    } = payload;

    println!("inside login route controller");
    println!("email{} , password {}", &email, &password);
    //TODO: validate email and password

    //fetch the user information
    let user_information =
        sqlx::query_as::<_, UserInformation>("SELECT * FROM user_information WHERE email = $1")
            .bind(email.trim())
            .fetch_one(&database)
            .await
            .unwrap();



    println!("{:#?}", &user_information);
}

///reset user password
pub async fn reset_password(Json(_payload): Json<UserInformation>) -> impl IntoResponse {
    //destructure the request body
    todo!()
}

//get the user profile
pub async fn user_profile(Json(_payload): Json<UserInformation>) -> impl IntoResponse {}

//update user profile
pub async fn update_user_profile(Json(_payload): Json<UserInformation>) -> impl IntoResponse {}
