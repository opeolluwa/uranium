use crate::models::users::EnumerateFields;
use crate::models::users::UserAuthCredentials;
use crate::models::users::UserInformation;
use crate::shared::api_response::ApiErrorResponse;
use crate::shared::api_response::ApiResponse;
use crate::shared::api_response::ApiSuccessResponse;
use crate::shared::jwt_schema::JwtPayload;
use crate::shared::jwt_schema::JwtSchema;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use bcrypt::verify;
use bcrypt::BcryptError;
use bcrypt::DEFAULT_COST;
use jsonwebtoken::Algorithm;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

// use futures::FutureExt;
// use serde_json::json;
// use sqlx::postgres::PgRow;
// use std::env;
// use validator::Validate;

///create a new user
pub async fn sign_up(
    Json(payload): Json<UserAuthCredentials>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<UserInformation>>), ApiErrorResponse> {
    //destructure the request body
    let UserAuthCredentials {
        fullname,
        password,
        username,
        email,
    } = &payload;

    //check through the fields to see that no field was badly formatted
    let entries = &payload.collect_as_strings();
    let mut bad_request_errors: Vec<String> = Vec::new();
    for (key, value) in entries {
        if value.is_empty() {
            let error = format!("{key} is empty");
            bad_request_errors.push(error);
        }
    }

    //if we have empty fields return error to client
    if !bad_request_errors.is_empty() {
        return Err(ApiErrorResponse::BadRequest {
            error: bad_request_errors,
        });
    }

    //generate id and hashed password
    let id = Uuid::new_v4();
    let hashed_password = bcrypt::hash(&password, DEFAULT_COST).unwrap();
    let new_user = sqlx::query_as::<_, UserInformation>(
        "INSERT INTO user_information (id, fullname, username, password, email) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(Some(id))
    .bind(Some(fullname))
    .bind(Some(username))
    .bind(Some(hashed_password))
    .bind(Some(email))
    .fetch_one(&database).await;

    // let new_user: Result<UserInformation, Err> = Ok(new_user);
    match new_user {
        Ok(result) => {
            //build the response
            let response: ApiSuccessResponse<UserInformation> =
                ApiSuccessResponse::<UserInformation> {
                    success: true,
                    message: String::from("missing email or password "),
                    data: Some(result),
                };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(err) => Err(ApiErrorResponse::ServerError {
            error: vec![err.to_string()],
        }),
    }
    // Ok(/* (axum::http::StatusCode, axum::Json<ApiSuccessResponse<UserInformation>>) */)
}

///login a new user
/// to login a user, fetch the request body and the database pool
/// use the pool to query the database for the user details in the request body
/// return result or error
/// Result<(StatusCode, Json<SuccessResponse<UserInformation>>), ErrorResponse<String>>
pub async fn login(
    Json(payload): Json<UserAuthCredentials>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<JwtPayload>>), ApiErrorResponse> {
    //destructure the payload to fetch user details
    let UserAuthCredentials {
        email, password, ..
    } = &payload;
    println!("inside login route controller");

    /*
     * validate the password and the email
     * if either is missing send error response
     */
    //check through the fields to see that no field was badly formatted
    let entries = &payload.collect_as_strings();
    let mut bad_request_errors: Vec<String> = Vec::new();
    for (key, value) in entries {
        if value.is_empty() {
            let error = format!("{key} is empty");
            bad_request_errors.push(error);
        }
    }

    //if we have empty fields return error to client
    if !bad_request_errors.is_empty() {
        return Err(ApiErrorResponse::BadRequest {
            error: bad_request_errors,
        });
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
            .await;

    let user = match user_information {
        Ok(user) => user,
        /*  Err(_) => UserInformation {
            id: Uuid::parse_str(""),
            fullname: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            username: "".to_string(),
        }, */
        Err(_) => todo!(),
    };


    
    //return the user data
    // (StatusCode::OK, Json(response))
    todo!()
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
