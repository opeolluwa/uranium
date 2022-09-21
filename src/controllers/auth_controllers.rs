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
        let response: ApiResponse<_, Vec<String>> = ApiResponse::<_, Vec<String>> {
            success: true,
            message: String::from("badly formatted input"),
            data: None::<UserInformation>,
            error: Some(bad_request_errors),
        };
        return Err(ApiErrorResponse::BadRequest);
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
    .fetch_one(&database)
    .await
    .unwrap();

    // let new_user: Result<UserInformation, Err> = Ok(new_user);
    //build the response
    let response: ApiSuccessResponse<UserInformation> = ApiSuccessResponse::<UserInformation> {
        success: true,
        message: String::from("missing email or password "),
        data: Some(new_user),
    };
    Ok((StatusCode::CREATED, Json(response)))
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
) -> impl IntoResponse {
    //destructure the payload to fetch user details
    let UserAuthCredentials {
        email, password, ..
    } = payload;
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
        id,
        email,
        fullname,
        password: hashed_password,
        ..
    } = user;

    // check password for correctness
    let is_correct_password = verify(password, &hashed_password);
    match is_correct_password {
        Ok(result) => {
            if !result {
                //build up response
                let response: ApiResponse<_, String> = ApiResponse::<_, String> {
                    success: true,
                    message: String::from("incorrect password"),
                    data: None,
                    error: None::<String>,
                };
                return (StatusCode::UNAUTHORIZED, Json(response));
            }
        }
        Err(error) => {
            //build up response
            let response: ApiResponse<_, String> = ApiResponse::<_, String> {
                success: false,
                message: String::from("incorrect password"),
                data: None,
                error: Some(BcryptError::InvalidCost(error.to_string()).to_string()),
            };
            // println!("{:?}", response);
            return (StatusCode::UNAUTHORIZED, Json(response));
        }
    }

    //:encrypt the user data
    let jwt_payload = JwtSchema {
        id: id.to_string(),
        email,
        fullname,
        exp: 2000000000, //may 2023
    };
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
        String::from("Ux6qlTEMdT0gSLq9GHp812R9XP3KSGSWcyrPpAypsTpRHxvLqYkeYNYfRZjL9")
    });
    //use a custom header
    let jwt_header = Header {
        alg: Algorithm::HS512,
        ..Default::default()
    };
    //build u the jwt token
    let token = encode(
        &jwt_header,
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
