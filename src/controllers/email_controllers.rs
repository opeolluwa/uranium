use crate::models::emails::EmailContext;
use crate::models::emails::EmailModel;
use crate::shared::api_response::ApiErrorResponse;
use crate::shared::api_response::ApiResponse;
use crate::shared::api_response::ApiSuccessResponse;
use crate::shared::mailer::parse_email_template;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use lettre::message::header;
use lettre::message::MultiPart;
use lettre::message::SinglePart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;
use once_cell::sync::Lazy;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

//the secrets
/// the once_cell create all us to add types lazily to variables with const and static binding
/// see documentation on <https://docs.rs/once_cell/latest/once_cell/>
static SMTP_USERNAME: Lazy<String> =
    Lazy::new(|| env::var("SMTP_USERNAME").expect("SMTP username not provided"));
static SMTP_PASSWORD: Lazy<String> =
    Lazy::new(|| env::var("SMTP_PASSWORD").expect("SMTP password not provided"));
static SMTP_HOST: Lazy<String> =
    Lazy::new(|| env::var("SMTP_HOST").expect("SMTP host not provided"));
static _SMTP_REPLY_TO_ADDRESS: Lazy<String> =
    Lazy::new(|| env::var("SMTP_PASSWORD").expect("SMTP reply-to-address not specified"));
static _SMTP_REPLY_TO_NAME: Lazy<String> =
    Lazy::new(|| env::var("SMTP_REPLY_TO_NAME").expect("SMTP reply-to-name not provided"));
static FRONTEND_URL: Lazy<String> = Lazy::new(|| {
    env::var("FRONTEND_URL").unwrap_or_else(|_| String::from("https://opeolluwa.verce.app"))
});

///send email handler
/// receive the user email, subject, fullname and message
/// call on lettre to dispatch the mail to the user
pub async fn send_email(
    Json(_payload): Json<EmailContext>,
    Extension(_database): Extension<PgPool>,
) -> impl IntoResponse {
    let content = r#"
    <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;">
                            You recently requested to reset your password for your
                            {{$data['product']}} account. Use the button below to
                            reset it. <strong style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box;">This
                              password reset is only valid for the next 24
                              hours.</strong>
                          </p>
    "#;
    let recipient_name = String::from("Adefemi");
    let email = Message::builder()
        .from("Nitride <opeolluwa@nitride.tld>".parse().unwrap())
        .reply_to("Yuin <opeolluwa@gmail.com>".parse().unwrap())
        .to("Hei <adefemiadeoye@yahoo.com>".parse().unwrap())
        .subject("Happy new year")
        .multipart(
            MultiPart::alternative() // This is composed of two parts.
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from("Hello from Lettre! A mailer library for Rust")), // Every message should have a plain text fallback.
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(parse_email_template(content.to_string(), recipient_name)),
                ),
        )
        .unwrap();

    let credentials = Credentials::new(SMTP_USERNAME.to_string(), SMTP_PASSWORD.to_string());

    // Open a remote connection to the smtp sever
    let mailer = SmtpTransport::relay(&SMTP_HOST)
        .unwrap()
        .credentials(credentials)
        .build();

    // Send the email
    let res = match mailer.send(&email) {
        Ok(_) => "Email sent successfully!".to_string(),
        Err(e) => format!("Could not send email: {:?}", e),
    };

    Json(res)
}

///receive email being sent from the portfolio
/// store it in the database
pub async fn receive_email(
    Json(payload): Json<EmailContext>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), ApiErrorResponse> {
    //destructure the email fields from the payload
    let EmailContext {
        fullname: sender_name,
        email: sender_email,
        subject: email_subject,
        message: email_body,
        ..
    } = &payload;

    //generate an  Id store the email in a database
    let id = Uuid::new_v4();
    let new_email =  sqlx::query_as::<_, EmailModel>(
        "INSERT INTO emails (id, sender_name, sender_email, email_subject, email_body) VALUES ($1, $2, $3, $4, $5)  RETURNING *",
    )
    .bind(Some(id))
    .bind(Some(sender_name))
    .bind(Some(sender_email))
    .bind(Some(email_subject))
    .bind(Some(email_body))
    .fetch_one(&database).await;

    /*
     * get the status of the received email
     * if successful, send an auto responder to the sender
     * if not return an error
     */
    match new_email {
        Ok(_) => {
            //send an auto response on success
            //TODO : dynamically get reply to email
            let from_email = format!("{sender_name} <{sender_email}>");
            let _reply_to = format!("{:?} <{:?}>", "adeoye", "adefemiadeoye@yahoo.com");
            let receiver_address = format!("{sender_name} <{sender_email}>");

            println!("{:#?}", &receiver_address);

            // todo!()
            //the auto response email content
            let email_content = format!(
                r#"
   
    
    
    <!--email content ---->
     <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;margin-top:15px;margin-bottom:15px">
     Thanks for reaching out, </br>
     Your email sent on <a href="{frontend_url}">{frontend_url}</a> has been received and will be attended to shortly.                
    </p>
    "#,
                frontend_url = FRONTEND_URL.to_lowercase()
            );

            //call on the template parser
            let message_content = parse_email_template(email_content, sender_name.to_string());
            let email = Message::builder()
                .from(from_email.parse().unwrap())
                .reply_to("adeoye <adefemiadeoye@yahoo.com>".parse().unwrap())
                .to(receiver_address.parse().unwrap())
                .subject(email_subject)
                .multipart(
                    MultiPart::alternative() // This is composed of two parts.
                        .singlepart(
                            SinglePart::builder()
                                .header(header::ContentType::TEXT_HTML)
                                .body(message_content),
                        ),
                )
                .unwrap();
            let credentials =
                Credentials::new(SMTP_USERNAME.to_string(), SMTP_PASSWORD.to_string());

            // Open a remote connection to the smtp sever
            let mailer =
                SmtpTransport::relay(&env::var("SMTP_HOST").expect("SMTP host not provided"))
                    .unwrap()
                    .credentials(credentials)
                    .build();

            // Send the email, if the mail is successful save it
            match mailer.send(&email) {
                Ok(_) => {
                    let response_body: ApiResponse<()> = ApiResponse::<()> {
                        success: true,
                        message: String::from("Message successfully sent"),
                        data: None,
                    };
                    //the response with ok status code and response body
                    Ok((StatusCode::OK, Json(response_body)))
                }
                Err(error_message) => Err(ApiErrorResponse::ConflictError {
                    message: error_message.to_string(),
                }),
            }
            //send the response back to the client application

            // todo!()
            // response
        }
        Err(error_message) => Err(ApiErrorResponse::ServerError {
            message: error_message.to_string(),
        }),
    }
}

///reply email
/// receive only the user email and subject and message
/// send the message to the user
pub async fn reply_email(
    Path(email_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<EmailContext>>), ApiErrorResponse> {
    let fetched_email = sqlx::query_as::<_, EmailContext>("SELECT * FROM emails WHERE id = $1")
        .bind(Some(email_id))
        .fetch_one(&database)
        .await;

    //return the fetched email
    match fetched_email {
        // if email is found, return the mail
        Ok(email) => {
            //build up the response
            let ok_response_body: ApiSuccessResponse<EmailContext> = ApiSuccessResponse {
                success: true,
                message: String::from("email successfully retrieved "),
                data: Some(email),
            };

            //return the response body
            Ok((StatusCode::OK, Json(ok_response_body)))
            // todo!()
        }
        //return a not found error
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            message: error_message.to_string(),
        }),
    }
}
///delete email
///receive the id of the mail to delete
///exec the query on the database
/// return result
pub async fn delete_email(
    Path(email_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<EmailContext>>), ApiErrorResponse> {
    let fetched_email = sqlx::query_as::<_, EmailContext>("SELECT * FROM emails WHERE id = $1")
        .bind(Some(email_id))
        .fetch_one(&database)
        .await;

    //return the fetched email
    match fetched_email {
        // if email is found, return the mail
        Ok(email) => {
            //build up the response
            let ok_response_body: ApiSuccessResponse<EmailContext> = ApiSuccessResponse {
                success: true,
                message: String::from("email successfully retrieved "),
                data: Some(email),
            };

            //return the response body
            Ok((StatusCode::OK, Json(ok_response_body)))
            // todo!()
        }
        //return a not found error
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            message: error_message.to_string(),
        }),
    }
}

///fetch email
/// retrieve an email from the data store
/// if the email was found, return the fond email else,
/// return a not found error
pub async fn fetch_email(
    Path(email_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<EmailContext>>), ApiErrorResponse> {
    let fetched_email = sqlx::query_as::<_, EmailContext>("SELECT * FROM emails WHERE id = $1")
        .bind(Some(email_id))
        .fetch_one(&database)
        .await;

    //return the fetched email
    match fetched_email {
        // if email is found, return the mail
        Ok(email) => {
            //build up the response
            let ok_response_body: ApiSuccessResponse<EmailContext> = ApiSuccessResponse {
                success: true,
                message: String::from("email successfully retrieved "),
                data: Some(email),
            };

            //return the response body
            Ok((StatusCode::OK, Json(ok_response_body)))
            // todo!()
        }
        //return a not found error
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            message: error_message.to_string(),
        }),
    }
}

///star email
/// mark email as important
pub async fn star_email(
    Path(email_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<EmailContext>>), ApiErrorResponse> {
    let fetched_email = sqlx::query_as::<_, EmailContext>("SELECT * FROM emails WHERE id = $1")
        .bind(Some(email_id))
        .fetch_one(&database)
        .await;

    //return the fetched email
    match fetched_email {
        // if email is found, return the mail
        Ok(email) => {
            //build up the response
            let ok_response_body: ApiSuccessResponse<EmailContext> = ApiSuccessResponse {
                success: true,
                message: String::from("email successfully retrieved "),
                data: Some(email),
            };

            //return the response body
            Ok((StatusCode::OK, Json(ok_response_body)))
            // todo!()
        }
        //return a not found error
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            message: error_message.to_string(),
        }),
    }
}
