use chrono::Datelike;
use lettre::message::{header, MultiPart, SinglePart};
// use lettre::transport::smtp::extension;
use lettre::{
    transport::smtp::authentication::Credentials,
    {Message, SmtpTransport, Transport},
};
use mailer_config::*;
use racoon_macros::racoon_error;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fs::File;

use std::io::prelude::*;

use std::{fmt};
use tinytemplate::TinyTemplate;

// the template file to be read will be as html unless otherwise  stated
// also the  template file fill be built as part of the source code
const TEMPLATE_FILE_EXTENSION: &str = ".html";

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

// the display trait allow easier debugging
// and formatting the MailService struct
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
    pub async fn send_email(
        &self,
        sender_address: &str,
        reply_to_addr: &str,
        email_body: &str,
    ) -> bool {
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
                            .body(String::from(&self.email_subject)), // Every message should have a plain text fallback.
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(parse_email_template(
                                email_body.to_string(),
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
    ///
    /// the function accept template name
    /// and reade the content as string
    pub fn parse_template<T>(
        &self,
        template_name: &str,
        template_context: &T,
    ) -> Result<String, Box<dyn Error>>
    where
        // impl Serialize for the concrete data
        // that will be used to saturate the template
        T: Serialize,
    {
        // get the template name
        // read the template content as string
        let template_file_path = format!(
            "{template_name}.{template_file_extension}",
            template_file_extension = TEMPLATE_FILE_EXTENSION
        );
        let mut template_content = String::new();
        let mut template_file = File::open(template_file_path)?;
        template_file.read_to_string(&mut template_content)?;

        // parse the content extracted
        let mut template_parser = TinyTemplate::new();
        let template_name = template_name.to_string(); // the template name required by the parser
        template_parser.add_template(template_name.trim(), &template_content)?;

        let rendered = template_parser.render(template_name.trim(), template_context)?;

        // parse the saturated data to the existing frame work
        let rendered = parse_email_template(rendered, self.recipient_name.clone());

        // println!("{}", rendered);

        // return the full stack template
        Ok(rendered)
    }
}

/// accept email content
/// add the content to the frame work
/// the content will return a full stack email template, already saturated with the required data
/// the returned string will be pasted to the SMTP client to dispatch the email
fn parse_email_template(email_content: String, recipient_name: String) -> String {
    let current_year: i32 = chrono::Utc::now().year();
    format!(
        r#"
<html xmlns="http://www.w3.org/1999/xhtml"><head><meta name="viewport" content="width=device-width,initial-scale=1"><meta http-equiv="Content-Type" content="text/html; charset=UTF-8"></head><body style="font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol';box-sizing:border-box;background-color:#f8fafc;color:#74787e;height:100%;hyphens:auto;line-height:1.4;margin:0;-moz-hyphens:auto;-ms-word-break:break-all;width:100%!important;-webkit-hyphens:auto;-webkit-text-size-adjust:none;word-break:break-word;font-size:16px"><table class="wrapper" style="font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol';box-sizing:border-box;background-color:#f8fafc;margin:0;padding:0;width:100%;-premailer-cellpadding:0;-premailer-cellspacing:0;-premailer-width:100%" role="presentation" width="100%" cellspacing="0" cellpadding="0"><tbody><tr><td style="font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol';box-sizing:border-box" align="unset"><table class="content" style="font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol';box-sizing:border-box;margin:0;padding:0;width:100%;-premailer-cellpadding:0;-premailer-cellspacing:0;-premailer-width:100%" role="presentation" width="100%" cellspacing="0" cellpadding="0"><tbody><tr><td class="header" style="font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol';box-sizing:border-box;padding:0;text-align:center;color:#f5f5f5;margin:0 auto"><a style="font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol';box-sizing:border-box;font-size:19px;font-weight:700;text-decoration:none;width:100%;height:20px;display:block;margin:0"></a></td></tr><tr><td style="font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol';box-sizing:border-box;padding:0;color:#101010;padding:20px 15px;margin:55px 0;line-height:26px"><b style="display:block">Hello {recipient_name},</b>{email_content}</td></tr><tr><p style="height:35px"></p><td class="footer" style="font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol';box-sizing:border-box;padding:0;text-align:center;color:#131212;margin-top:85px;background-color:#eee;padding:25px 15px;font-size:13px"><p style="font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol';box-sizing:border-box;font-size:19px;font-weight:700;text-decoration:none;width:100%;display:block;margin-bottom:3px;font-size:13px"><b>Racoon Mail Service</b></p><p style="width:65%;text-align:center;line-height:15px;margin:5px auto 45px">Racoon is an open-source enterprise-scale identity management system for microservices</p><p style="margin-top:5px"><a href="https://github.com/opeolluwa/racoon" style="text-decoration:none"><img width="24px" src="https://ik.imagekit.io/nethbooks/cdn/github_Jgna5WGU-.svg?ik-sdk-version=javascript-1.4.3&updatedAt=1677280933340" alt="github"></a><a href="" style="text-decoration:none"><img width="24px" src="https://ik.imagekit.io/nethbooks/cdn/twitter_VyuCE-cVG.svg?ik-sdk-version=javascript-1.4.3&updatedAt=1677280933299" alt="twitter"></a><a href="" style="text-decoration:none"><img width="24px" src="https://ik.imagekit.io/nethbooks/cdn/linkedin_UyIggF6qZ.svg?ik-sdk-version=javascript-1.4.3&updatedAt=1677280933450" alt="linkedin"></a></p><p style="font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol';box-sizing:border-box;line-height:1.5em;margin-top:0;font-size:13px;margin-top:10px;text-align:center">&copy; {current_year}<a href="https://github.com/opeolluwa/racoon" style="text-decoration:none">Racoon Mail Service</a>.<br><small style="display:block;margin-top:5px">All rights reserved &#124; The Racoon authors</small>.</p></td></tr></tbody></table></td></tr></tbody></table></body></html>
"#
    )
}
