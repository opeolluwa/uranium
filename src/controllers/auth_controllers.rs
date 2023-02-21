use crate::models::common::{EmailVerification, OneTimePassword};
use crate::models::emails::EmailPayload;
use crate::models::users::{AccountStatus, ResetUserPassword, UserInformation, UserModel};
use crate::utils::api_response::{ApiErrorResponse, ApiSuccessResponse, ValidatedRequest};
use crate::utils::jwt::JWT_SECRET;
use crate::utils::jwt::{set_jtw_exp, JwtClaims, JwtPayload};
use crate::utils::message_queue::MessageQueue;
use crate::utils::otp_handler::Otp;
use crate::utils::sql_query_builder::{Create, Find, FindByPk};
use axum::{http::StatusCode, Extension, Json};
use jsonwebtoken::{encode, Algorithm, Header};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::env;

const ACCESS_TOKEN_VALIDITY: u64 = 10; // the bearer token validity set to 10 minutes
const REFRESH_TOKEN_VALIDITY: u64 = 25; // 25 minutes for refresh token validity

/// create new user account
pub async fn sign_up(
    ValidatedRequest(payload): ValidatedRequest<UserInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    let new_user = UserModel::create(payload, &database).await;
    if let Err(error_message) = new_user {
        if error_message.to_string().to_lowercase()
            == *"no rows returned by a query that expected to return at least one row"
        {
            return Err(ApiErrorResponse::ServerError {
                message: String::from("A user with provided email already exists"),
            });
        }
        return Err(ApiErrorResponse::ServerError {
            message: error_message.to_string(),
        });
    }

    let user = new_user.ok().unwrap();
    /*
    destructure the user object,
    encrypt the data as JWt, send email to the user
    send the token back to the user as success response
    if error send the error response
    */
    let UserModel {
        id: user_id,
        email,
        fullname,
        ..
    } = &user;
    let jwt_payload = JwtClaims {
        id: user_id.to_string(),
        email: email.as_ref().unwrap().to_string(),
        fullname: fullname.as_ref().unwrap().to_string(),
        exp: set_jtw_exp(ACCESS_TOKEN_VALIDITY), //set expirations
    };

    // build the JWT Token and create a new token
    let jwt_token = jwt_payload.generate_token().unwrap();
    let generated_otp = Otp::new().save(&database).await;
    generated_otp.link_to_user(*user_id, &database).await;

    // send email to user
    let email_payload = EmailPayload {
        recipient_name: (&user.fullname.as_ref().unwrap()).to_string(),
        recipient_address: (&user.email.as_ref().unwrap()).to_string(),
        data: generated_otp.token.to_string(),
        email_subject: "new account".to_string(),
    };

    // add email to queue
    let queue_data = email_payload;
    let queue_name = env::var("EMAIL_QUEUE").expect("email queue name not specified");
    let new_queue = MessageQueue::new(queue_data, &queue_name);
    new_queue.enqueue();

    //build the response
    let response: ApiSuccessResponse<Value> = ApiSuccessResponse::<Value> {
        success: true,
        message: String::from("Please verify OTP send to your email to continue"),
        data: Some(json!({
            "token":jwt_token,
            "tokenType":"Bearer".to_string()
        })),
    };
    //return the response
    Ok((StatusCode::CREATED, Json(response)))
}

/// to verify email
/// retrieve the bearer token from the authorization header,
/// retrieve the otp from request body
/// validate token and updates account status
/// return error or success response
pub async fn verify_email(
    ValidatedRequest(payload): ValidatedRequest<OneTimePassword>,
    authenticated_user: JwtClaims,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    let user_information = UserModel::find_by_pk(&authenticated_user.id, &database).await;

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
            let is_valid_otp =
                Otp::validate_otp(user.otp_id.unwrap(), &payload.token, &database).await;
            if !is_valid_otp {
                return Err(ApiErrorResponse::BadRequest {
                    message: "invalid or expired OTP".to_string(),
                });
            }

            //update the user account status
            Otp::unlink_from_user(user.id, &database).await;
            let response_body = ApiSuccessResponse {
                success: true,
                message: String::from("User account successfully activated"),
                data: None,
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

pub async fn request_new_otp(
    // ValidatedRequest(payload): ValidatedRequest<EmailVerification>,
    authenticated_user: JwtClaims,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    // find the user
    let user_information = UserModel::find_by_pk(&authenticated_user.id, &database).await;
    if user_information.is_err() {
        return Err(ApiErrorResponse::BadRequest {
            message: String::from("A user with the provided email was not found!"),
        });
    }

    // generate new otp
    let user = user_information.ok().unwrap();
    let UserModel {
        id: user_id,
        email,
        fullname,
        ..
    } = &user;
    let jwt_payload = JwtClaims {
        id: user_id.to_string(),
        email: email.as_ref().unwrap().to_string(),
        fullname: fullname.as_ref().unwrap().to_string(),
        exp: set_jtw_exp(ACCESS_TOKEN_VALIDITY), //set expirations
    };

    // build the JWT Token and create a new token
    let jwt_token = jwt_payload.generate_token().unwrap();
    let generated_otp = Otp::new().save(&database).await;
    generated_otp.link_to_user(*user_id, &database).await;

    // send email to user
    let email_payload = EmailPayload {
        recipient_name: (&user.fullname.as_ref().unwrap()).to_string(),
        recipient_address: (&user.email.as_ref().unwrap()).to_string(),
        data: generated_otp.token.to_string(),
        email_subject: "Account verification token".to_string(),
    };

    // add email to queue
    let queue_data = email_payload;
    let queue_name = env::var("EMAIL_QUEUE").expect("email queue name not specified");
    let new_queue = MessageQueue::new(queue_data, &queue_name);
    new_queue.enqueue();

    //build the response
    let response: ApiSuccessResponse<Value> = ApiSuccessResponse::<Value> {
        success: true,
        message: String::from("Please verify OTP send to your email to continue"),
        data: Some(json!({
            // "user":UserModel { ..updated_user },
            "token":jwt_token,
            "tokenType":"Bearer".to_string()
        })),
    };
    Ok((StatusCode::CREATED, Json(response)))
}

/// request verification
/// Suppose a user account setup could not be completed during setup,
/// this handler let the user pick up from where he stopped
/// the user provide email, a JWT is generated, sent to the client interface/Application, An OTP is sent to the user.
pub async fn request_account_verification(
    ValidatedRequest(payload): ValidatedRequest<EmailVerification>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    // find the user
    let user_information = UserModel::find(json!({"email":payload.email }), &database).await;
    if user_information.is_err() {
        return Err(ApiErrorResponse::BadRequest {
            message: String::from("A user with the provided email was not found!"),
        });
    }

    if let Err(err_message) = user_information {
        return Err(ApiErrorResponse::BadRequest {
            message: err_message.to_string(), // message: String::from("A user with the provided email was not found!"),
        });
    }
    // generate new otp
    let user = user_information.ok().unwrap();
    let UserModel {
        id: user_id,
        email,
        fullname,
        ..
    } = &user;
    let jwt_payload = JwtClaims {
        id: user_id.to_string(),
        email: email.as_ref().unwrap().to_string(),
        fullname: fullname.as_ref().unwrap().to_string(),
        exp: set_jtw_exp(ACCESS_TOKEN_VALIDITY), //set expirations
    };

    // build the JWT Token and create a new token
    let jwt_token = jwt_payload.generate_token().unwrap();
    let generated_otp = Otp::new().save(&database).await;
    generated_otp.link_to_user(*user_id, &database).await;

    // send email to user
    let email_payload = EmailPayload {
        recipient_name: (&user.fullname.as_ref().unwrap()).to_string(),
        recipient_address: (&user.email.as_ref().unwrap()).to_string(),
        data: generated_otp.token.to_string(),
        email_subject: "Account verification token".to_string(),
    };

    // add email to queue
    let queue_data = email_payload;
    let queue_name = env::var("EMAIL_QUEUE").expect("email queue name not specified");
    let new_queue = MessageQueue::new(queue_data, &queue_name);
    new_queue.enqueue();

    //build the response
    let response: ApiSuccessResponse<Value> = ApiSuccessResponse::<Value> {
        success: true,
        message: String::from("Please verify OTP send to your email to continue"),
        data: Some(json!({
            "token":jwt_token,
            "tokenType":"Bearer".to_string()
        })),
    };
    Ok((StatusCode::CREATED, Json(response)))
}

///Login a New User :
/// to login a user, fetch the request body and the database pool
/// use the pool to query the database for the user details in the request body
/// return result or error
pub async fn login(
    ValidatedRequest(payload): ValidatedRequest<UserInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<JwtPayload>>), ApiErrorResponse> {
    let user_information = UserModel::find(json!({"email":payload.email}), &database).await;
    if let Err(error_message) = user_information {
        return Err(ApiErrorResponse::ServerError {
            message: error_message.to_string(),
        });
    }

    let user = user_information.ok().unwrap();
    let user_account_status = user.account_status.unwrap();

    //if user account has not been verified
    if user_account_status == AccountStatus::Inactive {
        return Err(ApiErrorResponse::Unauthorized {
            message: String::from(
                "Your account has not been activated. Please verify your email to continue",
            ),
        });
    }

    // if user account has been deactivated
    if user_account_status == AccountStatus::Deactivated {
        return Err(ApiErrorResponse::Unauthorized {
            message: String::from("Account has been suspended, please contact administrator"),
        });
    }

    //verify the password
    let is_correct_password: bool = user.verify_pswd_hash(&payload.password.unwrap());
    // racoon_debug!("{}", &is_correct_password);
    if !is_correct_password {
        return Err(ApiErrorResponse::Unauthorized {
            message: String::from("Invalid email or password"),
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

/// Get the user profile fom the database.
/// To do this,
///  Get the jwt token fom the header,
///  Validate the token then get the user_id from the validated token
/// - use the user_id to make request to the database
/// return the user details if no error else return the appropriate error code and response
pub async fn fetch_user_profile(
    authenticated_user: JwtClaims,
    Extension(database): Extension<PgPool>,
) -> Result<Json<ApiSuccessResponse<Value>>, ApiErrorResponse> {
    // Send the protected data to the user
    // fetch the user details from the database using...
    //the user id from the authenticated_user object
    let user_information =
        UserModel::find(json!({"email":authenticated_user.email.trim()}), &database).await;
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

pub async fn request_password_reset(
    ValidatedRequest(payload): ValidatedRequest<UserInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<JwtPayload>>), ApiErrorResponse> {
    let user_information = UserModel::find(json!({"email":payload.email}), &database).await;

    // check the error
    if let Err(error_message) = user_information {
        return Err(ApiErrorResponse::ServerError {
            message: error_message.to_string(),
        });
    }

    let user = user_information.ok().unwrap();
    // destructure the user if the password is correct
    let UserModel {
        id,
        email,
        fullname,
        ..
    } = &user;

    //encrypt the user data as JWT
    let jwt_payload = JwtClaims {
        id: id.to_string(),
        email: email.as_ref().unwrap().to_string(),
        fullname: fullname
            .as_ref()
            .unwrap_or(&"default".to_string())
            .to_string(),
        exp: set_jtw_exp(ACCESS_TOKEN_VALIDITY), //set expirations
    };
    let token = jwt_payload.generate_token().unwrap();
    let response: ApiSuccessResponse<JwtPayload> = ApiSuccessResponse::<JwtPayload> {
        success: true,
        message: String::from("Please verify OTP sent to yor email"),
        data: Some(JwtPayload {
            token,
            token_type: String::from("Bearer"),
        }),
    };
    Ok((StatusCode::OK, Json(response)))
}
/*
 * get the user details from the JWT claims
 * use the the extracted details to fetch the user data
 * send error if no user with the provided data was found
 * if found, update the password, expire the JWt,
 * generate new JWT. send new JWT to the client and a success response
 */
///reset user password
pub async fn reset_password(
    Json(payload): Json<ResetUserPassword>,
    authenticated_user: JwtClaims,
    Extension(database): Extension<PgPool>,
) -> Result<Json<ApiSuccessResponse<()>>, ApiErrorResponse> {
    let user_information =
        UserModel::find(json!({"email":authenticated_user.email.trim()}), &database).await;
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
