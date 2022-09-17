use crate::models::users::UserAuthCredentials;
use crate::models::users::UserInformation;
use crate::shared::api_response::ApiResponse;
use crate::shared::jwt_schema::JwtPayload;
use crate::shared::jwt_schema::JwtSchema;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use std::env;

// use futures::FutureExt;
// use bcrypt::{hash, verify};
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
    Json(payload): Json<UserAuthCredentials>,
    Extension(database): Extension<PgPool>,
) -> impl IntoResponse {
    //destructure the payload to fetch user details
    let UserAuthCredentials { email, password } = payload;
    println!("inside login route controller");

    /*
     * validate the password and the email
     * if either is missing send error response
     */
    if email.is_empty() || password.is_empty() {
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
    let UserInformation {
        email,
        fullname,
        username,
        ..
    } = user;
    //:encrypt the user data
    let jwt_payload = JwtSchema {
        id: String::from("c1961edd-6558-58f1-b56c-7931b93386a4"),
        email,
        fullname,
        username,
    };
    let jwt_secret = env::var("JWT_SECRET")
        .unwrap_or("Ux6qlTEMdT0gSLq9GHp812R9XP3KSGSWcyrPpAypsTpRHxvLqYkeYNYfRZjL9".to_string());
    let token = encode(
        &Header::default(),
        &jwt_payload,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .unwrap();

    //build up response
    let response: ApiResponse<JwtPayload, _> = ApiResponse::<JwtPayload, _> {
        success: true,
        message: String::from("user successfully retrieved"),
        data: Some(JwtPayload {
            token,
            token_type: String::from("Bearer"),
        }),
        error: None::<String>,
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
