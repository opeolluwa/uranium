use askama::Template;

#[derive(Template)]
#[template(path = "otp_code.html")]
pub struct OtpCodeTemplate<'a> {
    pub name: &'a str,
    pub code: &'a str,
}
