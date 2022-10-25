use crate::{
    models::users::{ResetUserPassword, UserAuthCredentials, UserInformation, UserModel},
    shared::{
        api_response::{ApiErrorResponse, ApiSuccessResponse, EnumerateFields, ValidatedRequest},
        jwt_schema::{set_jtw_exp, JwtClaims, JwtEncryptionKeys, JwtPayload},
        mailer::{send_email, EmailPayload},
        otp_handler::generate_otp,
    },
};
use axum::{http::StatusCode, Extension, Json};
use bcrypt::{verify, DEFAULT_COST};
use jsonwebtoken::{encode, Algorithm, Header};
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

///fetch the JWT defined environment and assign it's value to a life
/// call on the new method of JwtEncryption keys to accept and pass down the secret to the jsonwebtoken crate EncodingKey and DecodingKey modules
static JWT_SECRET: Lazy<JwtEncryptionKeys> = Lazy::new(|| -> JwtEncryptionKeys {
    let secret = std::env::var("JWT_SECRET").expect("Invalid or missing JWT Secret");
    JwtEncryptionKeys::new(secret.as_bytes())
});

/// the bearer token validity set to 1o minutes
const ACCESS_TOKEN_VALIDITY: u64 = 100;
/// refresh token set to 25 minutes
const REFRESH_TOKEN_VALIDITY: u64 = 25;

/// basic auth sign_up
// create new user account
pub async fn sign_up(
    ValidatedRequest(payload): ValidatedRequest<UserAuthCredentials>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    //check through the fields to see that no field was badly formatted
    let entries = &payload.collect_as_strings();
    let UserAuthCredentials { email, .. } = &payload;
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
            message: bad_request_errors.join(", "),
        });
    }

    /*
     * generate a UUID and hash the user password,
     * go on to save the hashed password along side other details
     * cat any error along the way
     */
    let id = Uuid::new_v4();
    let hashed_password = bcrypt::hash(&payload.password, DEFAULT_COST).unwrap();
    let new_user =  sqlx::query_as::<_, UserModel>(
        "INSERT INTO user_information (id, password, email) VALUES ($1, $2, $3) ON CONFLICT (email) DO NOTHING RETURNING *",
    )
    .bind(id)
    .bind(hashed_password)
    .bind(&email)
    .fetch_one(&database).await;

    // error handling
    match new_user {
        Ok(result) => {
            //generate a new otp and send email to the user
            let otp = generate_otp();
            let email_content = format!(
                r#"
             <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;">
                            
            Your account activation token is  <strong><em>{otp}<em></Strong>. <em style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box;">This
            Token is only valid for the next 5 minutes.</em>
            </p>
            "#,
            );

            let email_payload: EmailPayload = EmailPayload {
                recipient_name: "adefemi",
                recipient_address: "adefemiadeoye@yahoo.com",
                email_content,
                email_subject: "new account",
            };
            let sent_otp_to_user = send_email(email_payload);
            //build the response
            let response: ApiSuccessResponse<Value> = ApiSuccessResponse::<Value> {
                success: true,
                message: String::from("User account successfully created, please verify OTP send to your email to continue"),
                data: Some(
                    json!({
                        "user":UserModel { ..result },
                        "sentEmail":sent_otp_to_user
                    })
            )};
            //return the response
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(err) => Err(ApiErrorResponse::ConflictError {
            message: vec![
                err.to_string(),
                format!("an account with {email} already exists"),
            ]
            .join(", "),
        }),
    }
}
///create a new user
/// accept the user credentials,
/// check if user already exists
/// if not save the user
/// return success or error response
pub async fn _old_sign_up(
    Json(payload): Json<UserInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
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
            message: bad_request_errors.join(", "),
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
            let response: ApiSuccessResponse<Value> = ApiSuccessResponse::<Value> {
                success: true,
                message: String::from("User account successfully created"),
                data: Some(json!({"user":UserModel {
                    ..result // other fields
                }})),
            };
            //return the response
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(err) => Err(ApiErrorResponse::ConflictError {
            message: err.to_string(),
        }),
    }
}

///Login a New User :
/// to login a user, fetch the request body and the database pool
/// use the pool to query the database for the user details in the request body
/// return result or error
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
            message: bad_request_errors.join(", "),
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
            .bind(email)
            .fetch_one(&database)
            .await;

    match user_information {
        Ok(user) => {
            let stored_password = &user.password.as_ref().unwrap();
            let verify_password = verify(password, &stored_password);
            match verify_password {
                Ok(is_correct_password) => {
                    //send error if the password is not correct
                    if !is_correct_password {
                        return Err(ApiErrorResponse::WrongCredentials {
                            message: String::from("incorrect password"),
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
                        email: email.as_ref().unwrap().to_string(),
                        fullname: fullname.as_ref().unwrap().to_string(),
                        exp: set_jtw_exp(ACCESS_TOKEN_VALIDITY), //set expirations
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
                    message: error_message.to_string(),
                }),
            }
        }
        Err(error_message) => Err(ApiErrorResponse::ServerError {
            message: error_message.to_string(),
        }),
    }
}

/// Get the user profile fom the database.
/// To do this,
///  Get the jwt token fom the header,
///  Validate the token then get the user_id from the validated token
/// - use the user_id to make request to the database
/// return the user details if no error else return the appropriate error code and response
pub async fn user_profile(
    authenticated_user: JwtClaims,
    Extension(database): Extension<PgPool>,
) -> Result<Json<ApiSuccessResponse<Value>>, ApiErrorResponse> {
    // Send the protected data to the user
    // fetch the user details from the database using...
    //the user id from the authenticated_user object
    let user_information =
        sqlx::query_as::<_, UserModel>("SELECT * FROM user_information WHERE email = $1")
            .bind(Some(authenticated_user.email.trim()))
            .fetch_one(&database)
            .await;

    //handle errors
    match user_information {
        Ok(user_object) => {
            //build up the response body
            // don't return the value of the user password
            let response_body: ApiSuccessResponse<Value> = ApiSuccessResponse {
                success: true,
                message: "User information successfully fetched".to_string(),
                data: Some(json!({
                    "user":UserModel {
                    password: Some("".to_string()),
                    ..user_object
                }
                })),
            };

            Ok(Json(response_body))
        }
        Err(error_message) => Err(ApiErrorResponse::BadRequest {
            message: error_message.to_string(),
        }),
    }
}

/*
 * get the user details from the JWT claims
 * use the the extracted details to fetch the user data
 * send error if no user with the provided data was found
 *
 * if found, update the password, expire the JWt,
 * generate new JWT. send new JWT to the client and a success response
 */
///reset user password
pub async fn reset_password(
    Json(payload): Json<ResetUserPassword>,
    authenticated_user: JwtClaims,
    Extension(database): Extension<PgPool>,
) -> Result<Json<ApiSuccessResponse<()>>, ApiErrorResponse> {
    //check through the fields to see that no field was badly formatted
    let entries = &payload.collect_as_strings();
    let mut bad_request_errors: Vec<String> = Vec::new();
    for (key, value) in entries {
        if value.is_empty() {
            let error = format!("{key} is empty");
            bad_request_errors.push(error);
        }
    }

    if payload.new_password.trim() != payload.confirm_password {
        bad_request_errors.push("Password does not match".to_string());
    }

    //if we have empty fields return error to client
    if !bad_request_errors.is_empty() {
        return Err(ApiErrorResponse::BadRequest {
            message: bad_request_errors.join(", "),
        });
    }

    //destructure the payload
    let user_information =
        sqlx::query_as::<_, UserInformation>("SELECT * FROM user_information WHERE email = $1")
            .bind(Some(authenticated_user.email.trim()))
            .fetch_one(&database)
            .await;

    //handle errors
    match user_information {
        Ok(_) => {
            //update the user password
            let new_hashed_password =
                bcrypt::hash(payload.new_password, bcrypt::DEFAULT_COST).unwrap();
            sqlx::query_as::<_, UserInformation>(
                "UPDATE user_information SET password = $1 WHERE email = $2 RETURNING *",
            )
            .bind(Some(new_hashed_password.trim()))
            .bind(Some(authenticated_user.email.trim()))
            .fetch_one(&database)
            .await
            .unwrap();

            //build up the response body
            // don't return the value of the user password
            let response_body: ApiSuccessResponse<_> = ApiSuccessResponse {
                success: true,
                message: "User password successfully updated".to_string(),
                data: None,
            };
            //return the response
            Ok(Json(response_body))
        }
        Err(error_message) => Err(ApiErrorResponse::BadRequest {
            message: error_message.to_string(),
        }),
    }
}

/// Get the user profile fom the database.
/// to do this
///  Get the jwt token fom the header,
///  Validate the token then get the user_id from the validated token
///  go on to destructure the payload,
///  use SQL COALESCE($1, a)  to update the fields  
/// return the user details if no error else return the appropriate error code and response
pub async fn update_user_profile(
    Json(payload): Json<UserInformation>,
    authenticated_user: JwtClaims,
    Extension(database): Extension<PgPool>,
) -> Result<Json<ApiSuccessResponse<Value>>, ApiErrorResponse> {
    //get the user id from the destructured JWT claims
    //destructure the payload
    let user_information = sqlx::query_as::<_, UserInformation>(
        "UPDATE user_information SET email = COALESCE($1, email), username = COALESCE($2, username), fullname = COALESCE($3, fullname) WHERE id = $4 RETURNING *",
    )
    .bind(payload.email)
    .bind(payload.username)
    .bind(payload.fullname)
    .bind(sqlx::types::Uuid::parse_str(&authenticated_user.id).unwrap())
    .fetch_one(&database)
    .await;

    //handle errors
    match user_information {
        Ok(updated_user) => {
            //build up the response body
            // don't return the value of the user password
            let response_body: ApiSuccessResponse<Value> = ApiSuccessResponse {
                success: true,
                message: "User information successfully updated".to_string(),
                data: Some(json!({"user": UserInformation { ..updated_user }})),
            };
            //return the response
            Ok(Json(response_body))
        }
        Err(error_message) => Err(ApiErrorResponse::BadRequest {
            message: error_message.to_string(),
        }),
    }
}

/// get refresh token
pub async fn get_refresh_token(
    authenticated_user: JwtClaims,
    Extension(database): Extension<PgPool>,
) -> Result<Json<ApiSuccessResponse<JwtPayload>>, ApiErrorResponse> {
    // Send the protected data to the user
    // fetch the user details from the database using...
    //the user id from the authenticated_user object
    let user_information =
        sqlx::query_as::<_, UserModel>("SELECT * FROM user_information WHERE id = $1")
            .bind(sqlx::types::Uuid::parse_str(&authenticated_user.id).unwrap())
            .fetch_one(&database)
            .await;

    //handle errors
    match user_information {
        Ok(user_object) => {
            // destructure the user if the password is correct
            let UserModel {
                id,
                email,
                fullname,
                ..
            } = &user_object;

            //encrypt the user data
            //TODO: remove unwrap
            let jwt_payload = JwtClaims {
                id: id.to_string(),
                email: email.as_ref().unwrap().to_string(),
                fullname: fullname.as_ref().unwrap().to_string(),
                exp: set_jtw_exp(REFRESH_TOKEN_VALIDITY), //set expirations
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
            let response_body: ApiSuccessResponse<JwtPayload> = ApiSuccessResponse::<JwtPayload> {
                success: true,
                message: String::from("user authorization token successfully updated"),
                data: Some(JwtPayload {
                    token: token.unwrap(),
                    token_type: String::from("Refresh"),
                }),
            };
            Ok(Json(response_body))
        }
        Err(error_message) => Err(ApiErrorResponse::BadRequest {
            message: error_message.to_string(),
        }),
    }
}

// /// logout controller
// /// the logout controller will accept the bearer token via query params
// /// it will add the token to the auth_token table
// pub async fn logout(){

// }
