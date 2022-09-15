use crate::models::users::UserInformation;
use crate::shared::{jwt_schema::JwtSchema, user_schema::User};
use axum::{http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use std::env;
use validator::Validate;
///create a new user
pub async fn sign_up(Json(payload): Json<UserInformation>) -> impl IntoResponse {
    //destructure the request
    todo!();
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
pub async fn login(Json(payload): Json<UserInformation>) -> impl IntoResponse {
    //destructure the payload
    let UserInformation {
        email, password, ..
    } = payload;

    //validate username and password 
    todo!();
}

///reset user password
pub async fn reset_password(Json(payload): Json<UserInformation>) -> impl IntoResponse {
    //destructure the request body
    todo!()
}

//get the user profile
pub async fn user_profile(Json(_payload): Json<User>) -> impl IntoResponse {}

//update user profile
pub async fn update_user_profile(Json(_payload): Json<User>) -> impl IntoResponse {}
