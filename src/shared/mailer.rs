use chrono::Datelike;
use lettre::message::{header, MultiPart, SinglePart};
use lettre::{
    transport::smtp::authentication::Credentials,
    {Message, SmtpTransport, Transport},
};
use mailer_config::*;
use serde::{Deserialize, Serialize};
///the email server config
/// the configuration contains the following constants
/// 1. SMTP username
/// 2. SMTP password
/// 3. SMTP host
/// 4. SMTP reply to address
/// 5. SMTP reply to name
/// 6. FRONTEND URL
pub mod mailer_config {
    use once_cell::sync::Lazy;
    use std::env;

    pub static SMTP_USERNAME: Lazy<String> =
        Lazy::new(|| env::var("SMTP_USERNAME").expect("SMTP username not provided"));
    pub static SMTP_PASSWORD: Lazy<String> =
        Lazy::new(|| env::var("SMTP_PASSWORD").expect("SMTP password not provided"));
    pub static SMTP_HOST: Lazy<String> =
        Lazy::new(|| env::var("SMTP_HOST").expect("SMTP host not provided"));
    pub static SMTP_REPLY_TO_ADDRESS: Lazy<String> = Lazy::new(|| {
        env::var("SMTP_REPLY_TO_ADDRESS").expect("SMTP reply-to-address not specified")
    });
    pub static SMTP_REPLY_TO_NAME: Lazy<String> =
        Lazy::new(|| env::var("SMTP_REPLY_TO_NAME").expect("SMTP reply-to-name not provided"));
    pub static FRONTEND_URL: Lazy<String> = Lazy::new(|| {
        env::var("FRONTEND_URL").unwrap_or_else(|_| String::from("https://opeolluwa.verce.app"))
    });
}

///email payload
/// contains raw HTML to be injected into HTML template
/// the email subject,
/// the email recipient name
/// the email recipient address
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailPayload<'a> {
    pub recipient_name: &'a str,
    pub recipient_address: &'a str,
    pub email_content: String,
    pub email_subject: &'a str,
}
/// send email, accept the email body
/// dispatch the emil
// #[async_trait]
pub fn send_email(payload: EmailPayload) -> bool {
    // the receiver's address
    let recipient_address = format!(
        "{name}<{address}>",
        name = &payload.recipient_name.to_string(),
        address = &payload.recipient_address.to_string()
    );

    // the sender address from the SMTP configuration
    let sender_address = format!(
        "{name} <{address}>",
        name = *SMTP_REPLY_TO_NAME,
        address = *SMTP_REPLY_TO_ADDRESS
    );

    // println!("{}", &sender_address);
    // let sender_address = "Drizzles <no-reply@nitride.com>";
    //the email service builder
    let email = Message::builder()
        .from(sender_address.parse().unwrap())
        .reply_to("Drizzles <no-reply@nitride.com>".parse().unwrap())
        .to(recipient_address.parse().unwrap())
        .subject(payload.email_subject.to_string())
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
                        .body(parse_email_template(
                            payload.email_content.to_string(),
                            payload.recipient_name.to_string(),
                        )),
                ),
        )
        .unwrap();

    let credentials = Credentials::new(
        mailer_config::SMTP_USERNAME.to_string(),
        SMTP_PASSWORD.to_string(),
    );

    // Open a remote connection to the smtp sever
    let mailer = SmtpTransport::relay(&SMTP_HOST)
        .unwrap()
        .credentials(credentials)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => true,
        Err(err) => {
            println!("{:#?}", err);
            false
        }
    }
}

///accept template data
/// fill in the content
/// return the email body
pub fn parse_email_template(email_content: String, recipient_name: String) -> String {
    let current_year: i32 = chrono::Utc::now().year();
    format!(
        r#"
<html xmlns="http://www.w3.org/1999/xhtml">

<head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
</head>

<body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; background-color: #f8fafc; color: #74787e; height: 100%; hyphens: auto; line-height: 1.4; margin: 0; -moz-hyphens: auto; -ms-word-break: break-all; width: 100% !important; -webkit-hyphens: auto; -webkit-text-size-adjust: none; word-break: break-word; font-size:16px">
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

 <!-------------------------------inject logo here----------------->
     <div>
<img
    src="https://ik.imagekit.io/nethbooks/Nitride_1_-removebg-preview_hBxMjiRB8.png?ik-sdk-version=javascript-1.4.3&updatedAt=1671466966955"
    alt="nitride
    logo" style="
    width: 200px;

    margin: 10px
    auto ;text-align:
    center; display: flex; justify-content:
    center;
    align-items: center;justify-items: center;" />
  </div>

  <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 16px; line-height: 1.5em; margin-top: 0; text-align: left;">
    <!--introduction or salutation-->
    Hi <strong style="text-transform:capitalize">{recipient_name}</strong>
  </p>

     

  <div style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 18px; line-height: 1.5em; margin-top: 0; text-align: left;">
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
  <p style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; color: #3d4852; font-size: 14px; line-height: 1.5em; margin-top: 0; text-align: left;">
    Best Regards,<br />nitride team</p>
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
    &copy; {current_year} <a href="https://github.com/opeolluwa/nitride">nitride</a>.
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
