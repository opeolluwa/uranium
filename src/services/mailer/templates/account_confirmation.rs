use askama::Template;

#[derive(Template)]
#[template(path = "account_confirmation.html")]
pub struct AccountConfirmationTemplate<'a> {
    pub name: &'a str,
    pub code: Option<&'a str>,
    pub confirmation_link: Option<&'a str>,
}
