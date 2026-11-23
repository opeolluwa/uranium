use askama::Template;

#[derive(Template)]
#[template(path = "welcome.html")]
pub struct WelcomeTemplate<'a> {
    pub name: &'a str,
    pub dashboard_link: &'a str,
}
