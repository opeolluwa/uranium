use crate::{
    config::database::mongodb,
    shared::{jwt_schema::JwtSchema, user_schema::User},
};
use axum::{http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::doc;
use serde_json::json;
use std::env;
use validator::Validate;
///create a new user
pub async fn sign_up(Json(payload): Json<User>) -> impl IntoResponse {
    //destructure the request
    let User {
        firstname,
        lastname,
        email,
        password,
        ..
    } = payload;
    let database = mongodb().await;
    let collection = database.collection::<User>("user");

    //TODO: validate the user object, first check if user with email already exists
    // let error: Vec<String>;
    /*  if assert_eq!(firstname.is_empty(), true) {
        error.push("Firstname cannot be empty".to_string());
    } */

    /*
     * find user by email
     * if user already exist send error message
     * else create a new account with provided details
     */
    let user_already_exists = collection
        .find_one(doc! { "email": &email }, None)
        .await
        .unwrap();
    if let Some(_) = user_already_exists {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "success":false,
                "message":"a user with provided mail already exits",
                "data":None::<User>
            })),
        );
    }

    //construct a new user form the validated request payload
    let hashed_password = hash(password, 12).unwrap();
    let user = User {
        firstname: firstname,
        lastname: lastname,
        email: email,
        password: hashed_password,
    };

    //create new user
    collection.insert_one(&user, None).await.unwrap();
    (
        StatusCode::CREATED,
        Json(json!({
            "success":true,
            "message":"user successfully created".to_string(),
            "data":None::<User>
        })),
    )
}

///login a new user
pub async fn login(Json(payload): Json<User>) -> impl IntoResponse {
    //:validate the payload
    let mut request_body_errors: Vec<String> = vec![];
    if payload.password.is_empty() {
        request_body_errors.push("password cannot be blank".to_string());
    }

    if payload.email.is_empty() {
        request_body_errors.push("no email was provided ".to_string());
    }

    if request_body_errors.len() >= 1 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success":false,
            "message":"badly formatted request",
            "errors":request_body_errors
            })),
        );
    }
    //destructure the request body
    let User {
        email,
        password: user_password,
        ..
    } = payload;

    //find user by email
    let database = mongodb().await;
    let collection = database.collection::<User>("user");
    let result = collection
        .find_one(doc! { "email": &email }, None)
        .await
        .unwrap();

    //try to destructure the found object
    let (firstname, email, password) = if let Some(User {
        firstname,
        email,
        password,
        ..
    }) = result
    {
        (firstname, email, password)
    } else {
        //if no user was found return 404 error
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success":false,
                "message":"no use with provided credentials was found".to_string(),
                "data":None::<User>
            })),
        );
    };

    //check for correctness of password, if correct send access token
    let is_correct_password = verify(user_password, &password);
    // println!("the password result is {:?}", correct_password);
    match is_correct_password {
        Ok(_) => {
            //:encrypt the user data
            let jwt_payload = JwtSchema { email, firstname };
            let jwt_secret = env::var("JWT_SECRET").unwrap_or(
                "Ux6qlTEMdT0gSLq9GHp812R9XP3KSGSWcyrPpAypsTpRHxvLqYkeYNYfRZjL9".to_string(),
            );
            let token = encode(
                &Header::default(),
                &jwt_payload,
                &EncodingKey::from_secret(jwt_secret.as_bytes()),
            )
            .unwrap();

            //send the response
            (
                StatusCode::OK,
                Json(json!({
                    "success":true,
                    "message":"user successfully created".to_string(),
                    "data":json!({
                        "token":token,
                        "type":"Bearer".to_string()
                    })
                })),
            )
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success":false,
                "message":"invalid email or password".to_string(),
                "data":None::<User>
            })),
        ),
    }
}

///reset user password
pub async fn reset_password(Json(payload): Json<User>) -> impl IntoResponse {
    //destructure the request body
    let User { email, .. } = payload;
    Json(json!({
        "email":email,
    }))
}

//get the user profile
pub async fn user_profile(Json(_payload): Json<User>) -> impl IntoResponse {}

//update user profile
pub async fn update_user_profile(Json(_payload): Json<User>) -> impl IntoResponse {}
