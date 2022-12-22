use crate::models::emails::{EmailContext, EmailModel};
use crate::shared::api_response::{ApiErrorResponse, ApiResponse, ApiSuccessResponse};
use crate::shared::mailer::EmailPayload;
use crate::shared::mailer::{
    mailer_config::{FRONTEND_URL, SMTP_HOST, SMTP_PASSWORD, SMTP_USERNAME},
    parse_email_template, send_email as mail_dispatcher,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use lettre::message::{header, MultiPart, SinglePart};
use lettre::{
    transport::smtp::authentication::Credentials,
    {Message, SmtpTransport, Transport},
};
use sqlx::PgPool;
use uuid::Uuid;

//the secrets
/// the once_cell create all us to add types lazily to variables with const and static binding
/// see documentation on <https://docs.rs/once_cell/latest/once_cell/>

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
    //destructure the email fields from the  request payload
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
    .bind(&sender_name)
    .bind(sender_email)
    .bind(&email_subject)
    .bind(&email_body)
    .fetch_one(&database).await;

    //the auto response email content
    let sender_email_content = format!(
        r#"
    <!--email content ---->
     <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;margin-top:15px;margin-bottom:15px">
     Thanks for reaching out, </br>
     Your email sent on <a href="{frontend_url}">{frontend_url}</a> has been received and will be attended to shortly.                
    </p>
    "#,
        frontend_url = FRONTEND_URL.to_lowercase()
    );
    // dispatch the email
    let sender_email_payload: EmailPayload = EmailPayload {
        recipient_name: &sender_name,
        recipient_address: &sender_email,
        email_content: sender_email_content,
        email_subject: &email_subject,
    };

    // the receiver email
    //the auto response email content
    let receiver_email_content = format!(
        r#"
    <!--email content ---->
     <div style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;margin-top:15px;margin-bottom:15px">
     A new email was sent from <a href="{frontend_url}">{frontend_url}</a> by <strong>{sender_name}<{sender_email}></strong>                
    </div>

    <div style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;margin-top:15px;margin-bottom:15px">
    {email_body}        
    </div>

    "#,
        frontend_url = FRONTEND_URL.to_lowercase()
    );

    // dispatch the email
    let frontend_url: &str = &std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| String::from("https://opeolluwa.verce.app"));
    let receiver_email_subject = format!(" new email from {}", frontend_url);
    let receiver_email_payload: EmailPayload = EmailPayload {
        recipient_name: "Opeoluwa",
        recipient_address: "adefemiadeoye@yahoo.com",
        email_content: receiver_email_content,
        email_subject: &receiver_email_subject,
    };

    /*
     * get the status of the received email
     * if successful, send an auto responder to the sender
     * if not return an error
     */
    match new_email {
        Ok(_) => {
            //handle exception
            let sent_client_response: bool = mail_dispatcher(sender_email_payload);
            let sent_owner_response: bool = mail_dispatcher(receiver_email_payload);

            if !sent_client_response && !sent_owner_response {
                return Err(ApiErrorResponse::ServerError {
                    message: "An unexpected error was encountered, please try again later"
                        .to_string(),
                });
            }
            let response_body: ApiResponse<()> = ApiResponse::<()> {
                success: true,
                message: String::from("Message successfully sent"),
                data: None,
            };
            //the response with ok status code and response body
            Ok((StatusCode::OK, Json(response_body)))
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
