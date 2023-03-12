use chrono::Datelike;
use lettre::message::{header, MultiPart, SinglePart};
use lettre::{
    transport::smtp::authentication::Credentials,
    {Message, SmtpTransport, Transport},
};
use mailer_config::*;
use racoon_macros::racoon_error;
use serde::{Deserialize, Serialize};
use std::fmt;

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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MailService {
    pub recipient_name: String,
    pub recipient_address: String,
    pub email_subject: String,
    pub email_body: String,
}

impl fmt::Display for MailService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(recipient_name: {}\nrecipient_address: {}\nemail body: {}\nemail_subject: {})",
            self.recipient_name, self.recipient_address, self.email_body, self.email_subject
        )
    }
}

impl MailService {
    /// construct new email and returns the MailService struct
    /// #Example
    /// ```rust
    /// let name = "Opeoluwa";
    /// let address = "Opeolluwa@mailer.com";
    /// let subject = "test email";
    /// let body = "test body";
    ///
    /// let email = MailService::new(name,address,subject,body);
    /// ```
    pub fn new(name: &str, addr: &str, subj: &str, body: &str) -> Self {
        Self {
            recipient_name: name.to_string(),
            recipient_address: addr.to_string(),
            email_subject: subj.to_string(),
            email_body: body.to_string(),
        }
    }
    /// dispatch the emil
    /// #EXample
    /// ```rust
    ///let name = "Opeoluwa";
    /// let address = "Opeolluwa@mailer.com";
    /// let subject = "test email";
    /// let body = "test body";
    /// let email_service = MailService::new(name,address,subject,body);
    /// let sender_addr = "Drizzles <no-reply@nitride.com>";
    /// let reply_to_addr = "Drizzles <no-reply@nitride.com>";
    /// let is_mail_sent = email_service.send_email(sender_addr, reply_to_addr).await()
    /// println!("is the mail successfully sent? {is_mail_sent}");
    /// ```
    pub async fn send_email(&self, sender_address: &str, reply_to_addr: &str) -> bool {
        //email service builder
        let email = Message::builder()
            .from(sender_address.parse().unwrap())
            .reply_to(reply_to_addr.parse().unwrap())
            .to(self.recipient_address.parse().unwrap())
            .subject(self.email_subject.to_string())
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
                                "payload.data".to_string(),
                                self.recipient_name.clone(),
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
                racoon_error!("Could not send mail due to ");
                print!("{err:?}");
                false
            }
        }
    }

    /// parse email template
    /// get the template as string then extract the string content
    /// parse the string content
    /// saturate placeholders found in the template with concrete data
    /// return an html compatible string
    pub fn parse_template() {
        todo!()
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
<!--layout wrapper-->

<body
    style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; background-color: #f8fafc; color: #74787e; height: 100%; hyphens: auto; line-height: 1.4; margin: 0; -moz-hyphens: auto; -ms-word-break: break-all; width: 100% !important; -webkit-hyphens: auto; -webkit-text-size-adjust: none; word-break: break-word; font-size:16px">
    <table class="wrapper"
        style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; background-color: #f8fafc; margin: 0; padding: 0; width: 100%; -premailer-cellpadding: 0; -premailer-cellspacing: 0; -premailer-width: 100%;"
        role="presentation" width="100%" cellspacing="0" cellpadding="0">
        <tbody>
            <tr>
                <td style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box;"
                    align="unset">
                    <!--content wrapper-->
                    <table class="content"
                        style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; margin: 0; padding: 0; width: 100%; -premailer-cellpadding: 0; -premailer-cellspacing: 0; -premailer-width: 100%;"
                        role="presentation" width="100%" cellspacing="0" cellpadding="0">
                        <!-----------------------email content-------------->
                        <tbody>
                            <!-----------email header can contain logo or other brand element ----->
                            <tr>
                                <td class="header"
                                    style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; padding: 0; text-align: center; color: #f5f5f5; margin: 0px auto;">
                                    <a
                                        style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; font-size: 19px; font-weight: bold; text-decoration: none;  width: 100%; height:20px; display: block; margin: 0;">
                                        <!--logo goes here-->
                                    </a>
                                </td>
                            </tr>
                            <!--------email content or body wrapper, wrapper can contain mormal html-->
                            <tr>
                                <td
                                    style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; padding: 0; color: #101010; padding: 20px 15px; margin: 55px 0; line-height: 26px; ">

                                    <b>
                                        Hello {recipient_name},
                                    </b>
                                    {email_content}

                                </td>
                            </tr>
                            <!-----------------------email footer------------------>
                            <tr>
                                <p style="height:35px"></p>

                                <td class="footer"
                                    style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; padding: 0; text-align: center; color:#131212; margin-top: 85px; background-color: #eee; padding: 25px 15px; font-size: 13px;">
                                    <p
                                        style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; font-size: 19px; font-weight: bold; text-decoration: none;  width: 100%; display: block; margin-bottom: 3px;font-size: 13px;">
                                        <b>Racoon Mail Service</b>
                                    </p>
                                    <p style="width: 65%; text-align: center; margin:  auto;">
                                        Racoon is an enterprise scale identity management system for microservices
                                    </p>
                                    <!--social icons-->
                                    <p style="margin-top: 5px;">
                                        <a href="https://github.com/opeolluwa/racoon" style="text-decoration: none;">
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
                                                style="width:20px">
                                                <title>github</title>
                                                <path
                                                    d="M12,2A10,10 0 0,0 2,12C2,16.42 4.87,20.17 8.84,21.5C9.34,21.58 9.5,21.27 9.5,21C9.5,20.77 9.5,20.14 9.5,19.31C6.73,19.91 6.14,17.97 6.14,17.97C5.68,16.81 5.03,16.5 5.03,16.5C4.12,15.88 5.1,15.9 5.1,15.9C6.1,15.97 6.63,16.93 6.63,16.93C7.5,18.45 8.97,18 9.54,17.76C9.63,17.11 9.89,16.67 10.17,16.42C7.95,16.17 5.62,15.31 5.62,11.5C5.62,10.39 6,9.5 6.65,8.79C6.55,8.54 6.2,7.5 6.75,6.15C6.75,6.15 7.59,5.88 9.5,7.17C10.29,6.95 11.15,6.84 12,6.84C12.85,6.84 13.71,6.95 14.5,7.17C16.41,5.88 17.25,6.15 17.25,6.15C17.8,7.5 17.45,8.54 17.35,8.79C18,9.5 18.38,10.39 18.38,11.5C18.38,15.32 16.04,16.16 13.81,16.41C14.17,16.72 14.5,17.33 14.5,18.26C14.5,19.6 14.5,20.68 14.5,21C14.5,21.27 14.66,21.59 15.17,21.5C19.14,20.16 22,16.42 22,12A10,10 0 0,0 12,2Z" />
                                            </svg>
                                        </a>
                                        <a href="" style="text-decoration: none">
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
                                                style="width:20px">
                                                <title>twitter</title>
                                                <path
                                                    d="M22.46,6C21.69,6.35 20.86,6.58 20,6.69C20.88,6.16 21.56,5.32 21.88,4.31C21.05,4.81 20.13,5.16 19.16,5.36C18.37,4.5 17.26,4 16,4C13.65,4 11.73,5.92 11.73,8.29C11.73,8.63 11.77,8.96 11.84,9.27C8.28,9.09 5.11,7.38 3,4.79C2.63,5.42 2.42,6.16 2.42,6.94C2.42,8.43 3.17,9.75 4.33,10.5C3.62,10.5 2.96,10.3 2.38,10C2.38,10 2.38,10 2.38,10.03C2.38,12.11 3.86,13.85 5.82,14.24C5.46,14.34 5.08,14.39 4.69,14.39C4.42,14.39 4.15,14.36 3.89,14.31C4.43,16 6,17.26 7.89,17.29C6.43,18.45 4.58,19.13 2.56,19.13C2.22,19.13 1.88,19.11 1.54,19.07C3.44,20.29 5.7,21 8.12,21C16,21 20.33,14.46 20.33,8.79C20.33,8.6 20.33,8.42 20.32,8.23C21.16,7.63 21.88,6.87 22.46,6Z" />
                                            </svg>
                                        </a>
                                        <a href="" style="text-decoration: none;">
                                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
                                                style="width:20px ;text-decoration: none;">
                                                <title>linkedin</title>
                                                <path
                                                    d="M19 3A2 2 0 0 1 21 5V19A2 2 0 0 1 19 21H5A2 2 0 0 1 3 19V5A2 2 0 0 1 5 3H19M18.5 18.5V13.2A3.26 3.26 0 0 0 15.24 9.94C14.39 9.94 13.4 10.46 12.92 11.24V10.13H10.13V18.5H12.92V13.57C12.92 12.8 13.54 12.17 14.31 12.17A1.4 1.4 0 0 1 15.71 13.57V18.5H18.5M6.88 8.56A1.68 1.68 0 0 0 8.56 6.88C8.56 5.95 7.81 5.19 6.88 5.19A1.69 1.69 0 0 0 5.19 6.88C5.19 7.81 5.95 8.56 6.88 8.56M8.27 18.5V10.13H5.5V18.5H8.27Z" />
                                            </svg>
                                        </a>
                                    </p>
                                    <p
                                        style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol'; box-sizing: border-box; line-height: 1.5em; margin-top: 0; font-size: 13px; margin-top: 10px; text-align: center;">
                                        &copy; {current_year} <a href="https://github.com/opeolluwa/racoon" style="text-decoration: none;
                                        ">Racoon Mail
                                            Service</a>.<br />
                                        <small style="display: block; margin-top: 5px;">All rights reserved</small>.
                                    </p>
                                </td>
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
