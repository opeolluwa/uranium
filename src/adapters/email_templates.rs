use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum EmailTemplates {
    Signup,
    ForgottenPassword,
    Welcome,
    Default,
}

impl Default for EmailTemplates {
    fn default() -> Self {
        EmailTemplates::Default
    }
}
impl ToString for EmailTemplates {
    fn to_string(&self) -> String {
        match self {
            EmailTemplates::Signup => "sign-up".to_string(),
            EmailTemplates::ForgottenPassword => "forgotten-password".to_string(),
            EmailTemplates::Welcome => "welcome".to_string(),
            EmailTemplates::Default => "default".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct EmailBuilder {
    template: EmailTemplates,
    recipient: String,
    title: String,
    payload: HashMap<String, String>,
}

impl EmailBuilder {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            ..Default::default()
        }
    }

    pub fn use_template(self, template: EmailTemplates) -> Self {
        Self { template, ..self }
    }

    pub fn send_to(self, email: &str) -> Self {
        Self {
            recipient: email.to_string(),
            ..self
        }
    }

    pub fn with_payload(self, payload: HashMap<String, String>) -> Self {
        Self { payload, ..self }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Error serializing payload as bytes")
    }
}
