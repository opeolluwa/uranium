use crate::models::users::UserAuthCredentials;
use crate::models::users::UserInformation;
use crate::models::users::UserModel;
use crate::shared::api_response::ApiErrorResponse;
use crate::shared::api_response::ApiSuccessResponse;
use crate::shared::api_response::EnumerateFields;
use crate::shared::jwt_schema::JwtClaims;
use crate::shared::jwt_schema::JwtEncryptionKeys;
use crate::shared::jwt_schema::JwtPayload;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use bcrypt::verify;
use bcrypt::DEFAULT_COST;
use jsonwebtoken::Algorithm;
use jsonwebtoken::{encode, Header};
use once_cell::sync::Lazy;
use sqlx::PgPool;
use uuid::Uuid;

///fetch the JWT defined environment and assign it's value to a life
/// call on the new method of JwtEncryption keys to accept and pass down the secret to the jsonwebtoken crate EncodingKey and DecodingKey modules
static JWT_SECRET: Lazy<JwtEncryptionKeys> = Lazy::new(|| -> JwtEncryptionKeys {
    let secret = std::env::var("JWT_SECRET").expect("Invalid or missing JWT Secret");
    JwtEncryptionKeys::new(secret.as_bytes())
});

///create a new user
/// accept the user credentials,
/// check if user already exists
/// if not save the user
/// return success or error response
pub async fn sign_up(
    Json(payload): Json<UserInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<UserModel>>), ApiErrorResponse> {
    //destructure the HTTP request body
    let UserInformation {
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
            error: bad_request_errors.join(", ").to_string(),
        });
    }

    /*
     * generate a UUID and hash the user password,
     * go on to save the hashed password along side other details
     * cat any error along the way
     */
    let id = Uuid::new_v4();
    let hashed_password = bcrypt::hash(&password, DEFAULT_COST).unwrap();
    let new_user =  sqlx::query_as::<_, UserModel>(
        "INSERT INTO user_information (id, fullname, username, password, email) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (email) DO NOTHING RETURNING *",
    )
    .bind(Some(id))
    .bind(Some(fullname))
    .bind(Some(username))
    .bind(Some(hashed_password))
    .bind(Some(email))
    .fetch_one(&database).await;

    // error handling
    match new_user {
        Ok(result) => {
            //build the response
            let response: ApiSuccessResponse<UserModel> = ApiSuccessResponse::<UserModel> {
                success: true,
                message: String::from("User account successfully created"),
                data: Some(UserModel {
                    password: "".to_string(),
                    ..result // other fields
                }),
            };
            //return the response
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(err) => Err(ApiErrorResponse::ConflictError {
            error: vec![
                err.to_string(),
                format!("an account with {email} already exists"),
            ]
            .join(", ")
            .to_string(),
        }),
    }
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
            error: bad_request_errors.join(", ").to_string(),
        });
    }

    /*
     * if both email and password is provided ,
     * fetch the user information
     * if none if found send error else confirm the user password
     * if correct password, return jwt
     */
    let user_information =
        sqlx::query_as::<_, UserModel>("SELECT * FROM user_information WHERE email = $1")
            .bind(Some(email))
            .fetch_one(&database)
            .await;

    match user_information {
        Ok(user) => {
            let verify_password = verify(password, &user.password);
            match verify_password {
                Ok(is_correct_password) => {
                    //send error if the password is not correct
                    if !is_correct_password {
                        return Err(ApiErrorResponse::WrongCredentials {
                            error: String::from("incorrect password"),
                        });
                    }

                    // destructure the user if the password is correct
                    let UserModel {
                        id,
                        email,
                        fullname,
                        ..
                    } = &user;

                    //encrypt the user data
                    let jwt_payload = JwtClaims {
                        id: id.to_string(),
                        email: email.to_string(),
                        fullname: fullname.to_string(),
                        exp: 2000000000, //may 2023
                    };
                    //fetch the JWT secret
                    /*   let jwt_secret = crate::shared::jwt_schema::jwt_secret(); */
                    //use a custom header
                    let jwt_header = Header {
                        alg: Algorithm::HS512,
                        ..Default::default()
                    };

                    //build the user jwt token
                    let token = encode(&jwt_header, &jwt_payload, &JWT_SECRET.encoding);
                    //construct and return a response
                    let response: ApiSuccessResponse<JwtPayload> = ApiSuccessResponse::<JwtPayload> {
                        success: true,
                        message: String::from("user successfully logged in"),
                        data: Some(JwtPayload {
                            token: token.unwrap(),
                            token_type: String::from("Bearer"),
                        }),
                    };
                    // response
                    Ok((StatusCode::OK, Json(response)))
                }
                Err(error_message) => Err(ApiErrorResponse::BadRequest {
                    error: error_message.to_string(),
                }),
            }
        }
        Err(error_message) => Err(ApiErrorResponse::ServerError {
            error: error_message.to_string(),
        }),
    }
}

///get the user profile
/// to do this, get the jwt token fom the header,
/// validate the token
/// return the user details if no error else return the apt error code and response
// pub async fn user_profile(Json(_payload): Json<UserInformation>) -> impl IntoResponse {}

pub async fn user_profile(
    authenticated_user: JwtClaims,
    Extension(database): Extension<PgPool>,
) -> impl IntoResponse {
    // Send the protected data to the user
    // fetch the user details from the database using...
    //the user id from the authenticated_user object
    let user_information =
        sqlx::query_as::<_, UserInformation>("SELECT * FROM user_information WHERE id = $1")
            .bind(Some(authenticated_user.id))
            .fetch_one(&database)
            .await;

    //handle errors
    match user_information {
        Ok(user_object) => {
            //build up the response body
            let response_body: ApiSuccessResponse<UserInformation> = ApiSuccessResponse {
                success: true,
                message: "User information successfully fetched ".to_string(),
                data: Some(user_object),
            };

            Json(response_body)
        },
         Err(error_message) => {
            Err(ApiErrorResponse::BadRequest{error:error_message.to_string()})
        }
    }
}

///reset user password
pub async fn reset_password(Json(_payload): Json<UserInformation>) -> impl IntoResponse {
    //destructure the request body
    todo!()
}

//update user profile
pub async fn update_user_profile(Json(_payload): Json<UserInformation>) -> impl IntoResponse {}
