use crate::models::common::OneTimePassword;
use crate::models::users::{AccountStatus, ResetUserPassword, UserInformation, UserModel};
use crate::utils::api_response::{
    ApiErrorResponse, ApiSuccessResponse, EnumerateFields, ValidatedRequest,
};
use crate::utils::jwt::{set_jtw_exp, JwtClaims, JwtEncryptionKeys, JwtPayload};
use crate::utils::mailer::EmailPayload;
use crate::utils::message_queue::MessageQueue;
use crate::utils::otp_handler::{generate_otp, validate_otp};
use axum::{http::StatusCode, Extension, Json};
use bcrypt::{verify, DEFAULT_COST};
use jsonwebtoken::{encode, Algorithm, Header};
use once_cell::sync::Lazy;
use racoon_macros::debug_print;
use racoon_macros::debug_print::debug_print;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::env;
use uuid::Uuid;
///fetch the JWT defined environment and assign it's value to a life
/// call on the new method of JwtEncryption keys to accept and pass down the secret to the jsonwebtoken crate EncodingKey and DecodingKey modules
static JWT_SECRET: Lazy<JwtEncryptionKeys> = Lazy::new(|| -> JwtEncryptionKeys {
    let secret = std::env::var("JWT_SECRET").expect("Invalid or missing JWT Secret");
    JwtEncryptionKeys::new(secret.as_bytes())
});

/// the bearer token validity set to 10 minutes
const ACCESS_TOKEN_VALIDITY: u64 = 10;
/// refresh token set to 25 minutes
const REFRESH_TOKEN_VALIDITY: u64 = 25;

/// basic auth sign_up
// create new user account
pub async fn sign_up(
    ValidatedRequest(payload): ValidatedRequest<UserInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    let UserInformation {
        firstname,
        lastname,
        middlename,
        fullname,
        username,
        email,
        date_of_birth,
        gender,
        avatar,
        phone_number,
        password,
        ..
    } = payload;

    let sql_query = r#"
INSERT INTO
    user_information (
        id,gender,firstname,lastname,middlename,
        fullname,username,email, date_of_birth,avatar, phone_number,
        password
    ) VALUES
    ( $1, $2, NUllIF($3, ''), NUllIF($4, ''), NUllIF($5, ''),
        NUllIF($6, ''),NUllIF($7, ''), NUllIF($8, null),
        NUllIF($9, null), NUllIF($10, ''), NUllIF($11, ''), NULLIF($12, '')
    ) ON CONFLICT (email) DO NOTHING RETURNING *
    "#;
    /*
     * generate a UUID and hash the user password,
     * go on to save the hashed password along side other details
     * cat any error along the way
     */

    let id = Uuid::new_v4();
    let hashed_password = bcrypt::hash(password.unwrap_or_default().trim(), DEFAULT_COST).unwrap();
    let new_user = sqlx::query_as::<_, UserModel>(sql_query)
        .bind(id)
        .bind(gender.unwrap_or_default())
        .bind(firstname.unwrap_or_default())
        .bind(lastname.unwrap_or_default())
        .bind(middlename.unwrap_or_default())
        .bind(fullname.unwrap_or_default())
        .bind(username.unwrap_or_default())
        .bind(email.unwrap_or_default().trim())
        .bind(date_of_birth.unwrap_or_default())
        .bind(avatar.unwrap_or_default())
        .bind(phone_number.unwrap_or_default())
        .bind(hashed_password)
        .fetch_one(&database)
        .await;

    match new_user {
        Ok(user) => {
            // generate OTP and parse the email template
            let otp = generate_otp();
            println!("{otp}");
            let email_content = format!(
                r#"
        <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; line-height: 1.5em; margin-top: 0; text-align: left;">

        We are glad to have you on board with us. To complete your account set up, please use the OTP

        <h3 style="text-align:center">{otp}</h3>

        <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box;">This OTP is only valid for the next 5 minutes. If you did not request this OTP, please ignore this message.</p>
        </p>
        "#,
            );

            /*
            destructure the user object,
            encrypt the data as JWt, send email to the user
            send the token back to the user as success response
            if error send the error response
            */
            let UserModel {
                id,
                email,
                fullname,
                ..
            } = &user;
            // let fullname = &new_user.fullname.unwrap();

            let jwt_payload = JwtClaims {
                id: id.to_string(),
                email: email.as_ref().unwrap().to_string(),
                fullname: fullname.as_ref().unwrap().to_string(),
                exp: set_jtw_exp(ACCESS_TOKEN_VALIDITY), //set expirations
            };
            let jwt_token = jwt_payload.generate_token().unwrap();
            // send email to user
            let email_payload: EmailPayload = EmailPayload {
                recipient_name: (&user.fullname.as_ref().unwrap()).to_string(),
                recipient_address: (&user.email.as_ref().unwrap()).to_string(),
                email_content,
                email_subject: "new account".to_string(),
            };

            // add email to queue
            let queue_data = email_payload;
            let queue_name = env::var("EMAIL_QUEUE").expect("email queue name not specified");
            let new_queue: MessageQueue<EmailPayload> = MessageQueue::new(queue_data, &queue_name);
            new_queue.enqueue();

            //build the response
            let response: ApiSuccessResponse<Value> = ApiSuccessResponse::<Value> {
                success: true,
                message: String::from("Please verify OTP send to your email to continue"),
                data: Some(json!({
                    "user":UserModel { ..user },
                    "token":jwt_token,
                    "tokenType":"Bearer".to_string()
                })),
            };
            //return the response
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(error_message) => Err(ApiErrorResponse::ServerError {
            message: error_message.to_string(),
        }),
    }
}

///verify email
/// to verify email
/// retrieve the bearer token fo=rom the auth header,
/// retrieve the otp from request body
/// validate token and updates account status
/// return error or success response
pub async fn verify_email(
    ValidatedRequest(payload): ValidatedRequest<OneTimePassword>,
    // Json(payload): Json<OneTimePassword>,
    authenticated_user: JwtClaims,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    let user_information =
        sqlx::query_as::<_, UserModel>("SELECT * FROM user_information WHERE email = $1")
            .bind(&authenticated_user.email)
            .fetch_one(&database)
            .await;

    //handle errors
    match user_information {
        Ok(user) => {
            // if account has not been activated
            let user_account_status = user.account_status.unwrap();
            if user_account_status == AccountStatus::Active {
                return Err(ApiErrorResponse::ConflictError {
                    message: String::from("Email has already been verified"),
                });
            }

            // update the account status if the token is valid
            let is_valid_otp = validate_otp(&payload.token);
            if !is_valid_otp {
                return Err(ApiErrorResponse::BadRequest {
                    message: "invalid token".to_string(),
                });
            }

            //update th e user information
            sqlx::query_as::<_, UserInformation>(
                "UPDATE user_information SET account_status = $1 WHERE email = $2 RETURNING *",
            )
            .bind(AccountStatus::Active)
            .bind(Some(&authenticated_user.email.trim()))
            .fetch_one(&database)
            .await
            .unwrap();

            // build the response
            let response_body = ApiSuccessResponse {
                success: true,
                message: "User account successfully activated ".to_string(),
                data: Some(json!({
                    "user":UserModel {
                    password: Some("".to_string()),
                    account_status:Some(AccountStatus::Active),
                ..user
                }
                })),
            };

            Ok((StatusCode::OK, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::BadRequest {
            message: error_message.to_string(),
        }),
    }
}

/// request new token (OTP)
/// to request new OTP, it must be that the user account has not been confirmed
/// or in the case of password reset
/// at any rate, the token will accept email to return a JWT token to the user
/// the returned JWT will contain the required information needed by the server for further processing
/// the server will also send new token to the user's email if the email as found

pub async fn _request_new_token() {
    todo!()
}

///Login a New User :
/// to login a user, fetch the request body and the database pool
/// use the pool to query the database for the user details in the request body
/// return result or error
pub async fn _login(
    ValidatedRequest(payload): ValidatedRequest<UserInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<JwtPayload>>), ApiErrorResponse> {
    /*
     * if both email and password is provided ,
     * fetch the user information
     * if none if found send error else confirm the user password
     * if correct password, return jwt
     */
    let user_information =
        sqlx::query_as::<_, UserModel>("SELECT * FROM user_information WHERE email = $1")
            .bind(payload.email)
            .fetch_one(&database)
            .await;

    match user_information {
        Ok(user) => {
            // if account has not been activated
            let user_account_status = user.account_status.unwrap();
            if user_account_status == AccountStatus::Inactive {
                return Err(ApiErrorResponse::Unauthorized {
                    message: String::from("Please verify your email to continue"),
                });
            }

            // if user account has been deactivated
            if user_account_status == AccountStatus::Deactivated {
                return Err(ApiErrorResponse::Unauthorized {
                    message: String::from(
                        "Account ahs been suspended, please contact administrator",
                    ),
                });
            }

            let stored_password = &user.password.as_ref().unwrap();
            let verify_password = verify(payload.password.unwrap_or_default(), stored_password);
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
                        fullname: fullname
                            .as_ref()
                            .unwrap_or(&"default".to_string())
                            .to_string(),
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
                Err(_) => Err(ApiErrorResponse::BadRequest {
                    message: "Invalid username or password".to_string(),
                }),
            }
        }
        Err(_) => Err(ApiErrorResponse::ServerError {
            /* message: message.to_string(), */
            message: "account does not exist".to_string(),
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
    ValidatedRequest(payload): ValidatedRequest<UserInformation>,
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
