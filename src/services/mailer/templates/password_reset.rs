use askama::Template;

#[derive(Template)]
#[template(path = "password_reset.html")]
pub struct PasswordResetTemplate<'a> {
    pub name: &'a str,
    pub reset_link: &'a str,
}
