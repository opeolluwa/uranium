use crate::models::emails::EmailContext;
use crate::shared::api_response::ApiResponse;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Datelike;
use lettre::message::header;
use lettre::message::MultiPart;
use lettre::message::SinglePart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;
// use maud::{html, PreEscaped, DOCTYPE};
use std::env;
///send email
/// receive the user email, subject, fullname and message
/// call on lettre to dispatch the mail to the user
pub async fn send_email() -> impl IntoResponse {
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

    let credentials = Credentials::new(
        env::var("SMTP_USERNAME").expect("SMTP username not provided"),
        env::var("SMTP_PASSWORD").expect("SMTP password not provided"),
    );

    // Open a remote connection to the smtp sever
    let mailer = SmtpTransport::relay(&env::var("SMTP_HOST").expect("SMTP host not provided"))
        .unwrap()
        .credentials(credentials)
        .build();

    // Send the email
    let res = match mailer.send(&email) {
        Ok(_) => format!("Email sent successfully!"),
        Err(e) => format!("Could not send email: {:?}", e),
    };

    Json(res)
}

///receive email being sent from the portfolio
/// store it in the database
pub async fn receive_email(Json(payload): Json<EmailContext>) -> impl IntoResponse {
    //destructure the email fields from the payload
    let EmailContext {
        fullname: sender_name,
        email: sender_email,
        subject: email_subject,
        message: message_content,
    } = payload;

    //format email sender name
    let from_email = format!("{sender_name} <{sender_email}>");
    let reply_to = format!("{sender_name} <{sender_email}>");
    /*    let email_content = html!(
        (DOCTYPE)
        p{"Hey, you have a new email from (sender_name) "}
        p {"See the the message content below"}
        p { "(message_content)"}
    ); */

    let email_content = format!(
        r#"
    <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;">
    You have a new email from {sender_email}. See the message content below                      
    </p>

    
    
    <!--email content ---->
     <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;margin-top:15px;margin-bottom:15px">
     <strong>&#34;<strong><em>{message_content}</em><strong>&#34;<strong>                     
    </p>
    "#
    );

    //call on the template parser
    let message_content = parse_email_template(email_content, "Opeoluwa".to_string());

    let email = Message::builder()
        .from(from_email.parse().unwrap())
        .reply_to(reply_to.parse().unwrap())
        .to("Hei <adefemiadeoye@yahoo.com>".parse().unwrap())
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

    let credentials = Credentials::new(
        env::var("SMTP_USERNAME").expect("SMTP username not provided"),
        env::var("SMTP_PASSWORD").expect("SMTP password not provided"),
    );

    // Open a remote connection to the smtp sever
    let mailer = SmtpTransport::relay(&env::var("SMTP_HOST").expect("SMTP host not provided"))
        .unwrap()
        .credentials(credentials)
        .build();

    // Send the email
    let res: ApiResponse<_, _> = match mailer.send(&email) {
        Ok(_) => ApiResponse::<(), ()> {
            success: true,
            message: String::from("Message successfully sent"),
            data: None,
            error: None,
        },
        Err(error_message) => ApiResponse::<_, _> {
            success: false,
            message: format!("failed to send message due to {:?}", error_message),
            data: None,
            error: None,
        },
    };

    Json(res)
}

///reply email
/// receive only the user email and subject and message
/// send the message to the user
pub async fn reply_email() -> impl IntoResponse {}

///delete email
///receive the id of the mail to delete
///exec the query on the database
/// return result
pub async fn delete_email() -> impl IntoResponse {}

///fetch email
/// retrieve an email from the data store
pub async fn fetch_email() -> impl IntoResponse {}

///star email
/// mark email as important
pub async fn star_email() -> impl IntoResponse {}

///accept template data
/// fill in the content
/// return the email body
fn parse_email_template(email_content: String, recipient_name: String) -> String {
    let current_year: i32 = chrono::Utc::now().year();
    format!(
        r#"
<html xmlns="http://www.w3.org/1999/xhtml">

<head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
</head>

<body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; background-color: #f8fafc; color: #74787e; height: 100%; hyphens: auto; line-height: 1.4; margin: 0; -moz-hyphens: auto; -ms-word-break: break-all; width: 100% !important; -webkit-hyphens: auto; -webkit-text-size-adjust: none; word-break: break-word;">
  <table class="wrapper" style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; background-color: #f8fafc; margin: 0; padding: 0; width: 100%; -premailer-cellpadding: 0; -premailer-cellspacing: 0; -premailer-width: 100%;" role="presentation" width="100%" cellspacing="0" cellpadding="0">
    <tbody>
      <tr>
        <td style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box;" align="center">
          <table class="content" style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; margin: 0; padding: 0; width: 100%; -premailer-cellpadding: 0; -premailer-cellspacing: 0; -premailer-width: 100%;" role="presentation" width="100%" cellspacing="0" cellpadding="0">
            <tbody>
              <tr>
                <td class="header" style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; padding: 25px 0; text-align: center;">
                  <a style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #bbbfc3; font-size: 19px; font-weight: bold; text-decoration: none; text-shadow: 0 1px 0 white;">
                  </a>
                </td>
              </tr>

              <!-- Email Body -->
              <tr>
                <td class="body" style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; background-color: #ffffff; border-bottom: 1px solid #edeff2; border-top: 1px solid #edeff2; margin: 0; padding: 0; width: 100%; -premailer-cellpadding: 0; -premailer-cellspacing: 0; -premailer-width: 100%;" width="100%">
                  <table class="inner-body" style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; background-color: #ffffff; margin: 0 auto; padding: 0; width: 570px; -premailer-cellpadding: 0; -premailer-cellspacing: 0; -premailer-width: 570px;" role="presentation" width="570" cellspacing="0" cellpadding="0" align="center">

                    <!-- Body content -->
                    <tbody>
                      <tr>
                        <td class="content-cell" style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; padding: 35px;">
                          <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;">
                            <!--introduction or salutation-->
                            Hi <strong>{recipient_name}</strong>
                          </p>

                          <div style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;">
                            <!------------------inject email content here=-------------------->
                            {email_content}
                          </div>

                          <table class="action" style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; margin: 30px auto; padding: 0; text-align: center; width: 100%; -premailer-cellpadding: 0; -premailer-cellspacing: 0; -premailer-width: 100%;" role="presentation" width="100%" cellspacing="0" cellpadding="0" align="center">
                            <tbody>
                              <tr>
                                <td style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box;" align="center">
                                  <table style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box;" role="presentation" border="0" width="100%" cellspacing="0" cellpadding="0">
                                    <tbody>
                                      <tr>

                                        <td style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box;" align="center">
                                          <table style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box;" role="presentation" border="0" cellspacing="0" cellpadding="0">
                                            <tbody>
                                              <tr>
                                                <td style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box;">

                                                </td>
                                              </tr>
                                            </tbody>
                                          </table>
                                        </td>
                                      </tr>
                                    </tbody>
                                  </table>
                                </td>
                              </tr>
                            </tbody>
                          </table>
                          <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;">
                            &nbsp;</p>
                          <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;">
                            Best Regards,<br />Opeoluwa</p>
                          <hr style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box;" />
                          <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;">
                            <small style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box;">If
                              you&rsquo;re having trouble with the button above,
                              copy and paste the URL below into your web
                              browser.</small>
                          </p>

                        </td>
                      </tr>
                    </tbody>
                  </table>
                </td>
              </tr>
              <tr>
                <td style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box;">
                  <table class="footer" style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; margin: 0 auto; padding: 0; text-align: center; width: 570px; -premailer-cellpadding: 0; -premailer-cellspacing: 0; -premailer-width: 570px;" role="presentation" width="570" cellspacing="0" cellpadding="0" align="center">
                    <tbody>
                      <tr>
                        <td class="content-cell" style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; padding: 35px;" align="center">
                          <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; line-height: 1.5em; margin-top: 0; color: #aeaeae; font-size: 12px; text-align: center;">
                            &copy; {current_year} <a href="https://www.linkedin.com/in/adefemi-adeoye">Opeoluwa</a>.
                            All rights reserved.</p>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </td>
              </tr>
            </tbody>
          </table>
        </td>
      </tr>
    </tbody>
  </table>
</body>

</html>
"#
    )
}
