use crate::models::emails::{EmailContext, EmailFolder, EmailModel};
use crate::shared::api_response::{
    ApiErrorResponse, ApiResponse, ApiSuccessResponse, Pagination, ValidatedRequest,
};
use crate::shared::mailer::EmailPayload;
use crate::shared::mailer::{mailer_config::FRONTEND_URL, send_email as mail_dispatcher};
use axum::extract::Query;
use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

///get all emails by pagination
/// default to default pagination config
pub async fn get_all_emails(
    pagination: Option<Query<Pagination>>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    // try and get the quey params or deflect to default
    // let pagination_params = query_params;
    let Query(pagination) = pagination.unwrap_or_default();
    let Pagination {
        page: current_page,
        no_of_rows,
    } = &pagination;
    let limit = (current_page - 1) * no_of_rows;

    //get the emails from the database
    let fetched_emails = sqlx::query_as::<_, EmailModel>(
        "SELECT * FROM emails ORDER BY date_sent DESC LIMIT $1 OFFSET $2",
    )
    .bind(no_of_rows)
    .bind(limit)
    .fetch_all(&database)
    .await
    .ok();

    let Some(fetched_email_array) = fetched_emails else {
        return Err(ApiErrorResponse::NotFound {
            message: "Error fetching emails".to_string(),
        })
    };

    //build the  body
    let response_body: ApiSuccessResponse<Value> = ApiSuccessResponse {
        success: true,
        message: String::from("email successfully fetched"),
        data: Some(json!({
        "emails": fetched_email_array,
        "currentPage" : &pagination.page.to_string(),
        "noOfRows":&pagination.no_of_rows.to_string()
        })),
    };
    //return the response with 200 status code
    Ok((StatusCode::OK, Json(response_body)))
}

///send email handler
/// receive the user email, subject, fullname and message
/// call on lettre to dispatch the mail to the user
pub async fn send_email(
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
        "INSERT INTO emails (id, sender_name, sender_email, email_subject, email_body, folder) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )

    .bind(Some(id))
    .bind(sender_name)
    .bind(sender_email)
    .bind(email_subject)
    .bind(email_body)
    .bind("Sent")
    .fetch_one(&database).await;

    //the auto response email content
    let receiver_email_content = format!(
        r#"
    <!--email content ---->
    <div style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;margin-top:15px;margin-bottom:15px">
    {email_body}        
    </div>
"#
    );

    // dispatch the email
    let frontend_url: &str = &std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| String::from("https://opeolluwa.verce.app"));
    let receiver_email_subject = format!(" new email from {}", frontend_url);
    let receiver_email_payload: EmailPayload = EmailPayload {
        recipient_name: sender_name,
        recipient_address: sender_email,
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
            let sent_owner_response: bool = mail_dispatcher(receiver_email_payload);
            if !sent_owner_response {
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
    .bind(sender_name)
    .bind(sender_email)
    .bind(email_subject)
    .bind(email_body)
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
        recipient_name: sender_name,
        recipient_address: sender_email,
        email_content: sender_email_content,
        email_subject,
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
    ValidatedRequest(payload): ValidatedRequest<EmailContext>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<EmailContext>>), ApiErrorResponse> {
    //destructure the email fields from the  request payload
    let EmailContext {
        message: email_body,
        ..
    } = &payload;

    //the email content
    let receiver_email_content = format!(
        r#"
    <!--email content ---->
    <div style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;margin-top:15px;margin-bottom:15px">
    <hr style="margin:10px 0"></hr>
    {email_body}        
    </div>
"#
    );

    // the fetched email which we want to reply to
    let fetched_email = sqlx::query_as::<_, EmailContext>("SELECT * FROM emails WHERE id = $1")
        .bind(Some(email_id))
        .fetch_one(&database)
        .await;

    //return the fetched email
    match fetched_email {
        // if email is found, return the mail
        Ok(email) => {
            let email_payload: EmailPayload = EmailPayload {
                recipient_name: &email.fullname,
                recipient_address: &email.email,
                email_content: receiver_email_content,
                email_subject: &("Reply: ".to_owned() + &email.subject),
            };

            //handle exception
            let sent_owner_response: bool = mail_dispatcher(email_payload);
            if !sent_owner_response {
                return Err(ApiErrorResponse::ServerError {
                    message: "An unexpected error was encountered, please try again later"
                        .to_string(),
                });
            }

            //build up the response
            let ok_response_body: ApiSuccessResponse<EmailContext> = ApiSuccessResponse {
                success: true,
                message: String::from("email successfully sent "),
                data: None,
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

///unstRstar email
/// receive the email id as path variable
/// find the email
/// star it
/// return the update or error
pub async fn un_star_email(
    Path(email_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<EmailModel>>), ApiErrorResponse> {
    let starred_email = sqlx::query_as::<_, EmailModel>(
        "UPDATE emails SET is_starred = $1 WHERE id = $2 RETURNING *",
    )
    .bind(false)
    .bind(Some(email_id))
    .fetch_one(&database)
    .await
    .ok();

    //handle exception
    let Some(email) = starred_email else {
        return Err(ApiErrorResponse::NotFound { message: "email not found".to_string() });
    };

    let response_body: ApiSuccessResponse<EmailModel> = ApiSuccessResponse {
        success: true,
        message: String::from("email successfully un-starred "),
        data: Some(email),
    };

    //return the response body
    Ok((StatusCode::OK, Json(response_body)))
}

///delete email
///receive the id of the mail to delete
///exec the query on the database
/// return result
pub async fn delete_email(
    Path(email_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<EmailModel>>), ApiErrorResponse> {
    let deleted_email =
        sqlx::query_as::<_, EmailModel>("UPDATE emails SET folder = $1 WHERE id = $2 RETURNING *")
            .bind(EmailFolder::Trash)
            .bind(Some(email_id))
            .fetch_one(&database)
            .await
            .ok();

    //handle exception
    let Some(email) =deleted_email else {
        return Err(ApiErrorResponse::NotFound { message: "email not found".to_string() });
    };

    let response_body: ApiSuccessResponse<EmailModel> = ApiSuccessResponse {
        success: true,
        message: String::from("email successfully deleted"),
        data: Some(email),
    };

    //return the response body
    Ok((StatusCode::OK, Json(response_body)))
}

///fetch email
/// retrieve an email from the data store
/// if the email was found, return the fond email else,
/// return a not found error
pub async fn get_email_by_id(
    Path(email_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<EmailModel>>), ApiErrorResponse> {
    let fetched_email = sqlx::query_as::<_, EmailModel>("SELECT * FROM emails WHERE id = $1")
        .bind(Some(email_id))
        .fetch_one(&database)
        .await;

    //return the fetched email
    match fetched_email {
        // if email is found, return the mail
        Ok(email) => {
            //build up the response
            let ok_response_body: ApiSuccessResponse<EmailModel> = ApiSuccessResponse {
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
/// receive the email id as path variable
/// find the email
/// star it
/// return the update or error
pub async fn star_email(
    Path(email_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<EmailModel>>), ApiErrorResponse> {
    let starred_email = sqlx::query_as::<_, EmailModel>(
        "UPDATE emails SET is_starred = $1 WHERE id = $2 RETURNING *",
    )
    .bind(true)
    .bind(Some(email_id))
    .fetch_one(&database)
    .await
    .ok();

    //handle exception
    let Some(email) = starred_email else {
        return Err(ApiErrorResponse::NotFound { message: "email not found".to_string() });
    };

    let response_body: ApiSuccessResponse<EmailModel> = ApiSuccessResponse {
        success: true,
        message: String::from("email successfully starred "),
        data: Some(email),
    };

    //return the response body
    Ok((StatusCode::OK, Json(response_body)))
}

///move email to archive
/// receive the email id as path variable
/// find the email
/// star it
/// return the update or error
pub async fn archive_email(
    Path(email_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<EmailModel>>), ApiErrorResponse> {
    let archive_email = sqlx::query_as::<_, EmailModel>(
        "UPDATE emails SET is_archived = $1 WHERE id = $2 RETURNING *",
    )
    .bind(true)
    .bind(Some(email_id))
    .fetch_one(&database)
    .await
    .ok();

    //handle exception
    let Some(email) = archive_email else {
        return Err(ApiErrorResponse::NotFound { message: "email not found".to_string() });
    };

    let response_body: ApiSuccessResponse<EmailModel> = ApiSuccessResponse {
        success: true,
        message: String::from("email successfully archived"),
        data: Some(email),
    };

    //return the response body
    Ok((StatusCode::OK, Json(response_body)))
}
