use crate::models::users::UserInformation;
use crate::shared::api_response::ApiResponse;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use sqlx::PgPool;

// use futures::FutureExt;
// use bcrypt::{hash, verify};
// use jsonwebtoken::{encode, EncodingKey, Header};
// use serde_json::json;
// use sqlx::postgres::PgRow;
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

    /*
     * validate the password and the email
     * if either is missing send error response
     */
    if email == None || password == None {
        let response: ApiResponse<_, _> = ApiResponse::<_, _> {
            success: true,
            message: String::from("missing email or password "),
            data: None,
            error: None,
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    /*
     * if both email and password is provided ,
     * fetch the user information
     * if none if found send error else confirm the user password
     * if correct password, return jwt
     */
    let user_information =
        sqlx::query_as::<_, UserInformation>("SELECT * FROM user_information WHERE email = $1")
            .bind(Some(email))
            .fetch_one(&database)
            .await
            .unwrap();

    //move query result to a new variable
    let user: UserInformation = user_information;
    //build up response
    let response: ApiResponse<UserInformation, _> = ApiResponse::<UserInformation, _> {
        success: true,
        message: String::from("user successfully retrieved"),
        data: Some(user),
        error: None::<UserInformation>,
    };

    //return the user data
    (StatusCode::OK, Json(response))
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
