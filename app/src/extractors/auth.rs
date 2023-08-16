use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum UserGender {
    Male,
    Female,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]

pub struct CreateUser {
    // pub id: Uuid,
    #[validate(required, length(min = 1))]
    pub firstname: Option<String>,
    #[validate(required, length(min = 1))]
    pub lastname: Option<String>,
    #[validate(required, length(min = 1))]
    pub middlename: Option<String>,
    #[validate(required, length(min = 1))]
    pub fullname: Option<String>,
    #[validate(required, length(min = 1))]
    pub username: Option<String>,
    #[validate(required, email(message = "please use a valid email"))]
    pub email: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<UserGender>,
    #[validate(url(message = "invalid URL detected"))]
    pub avatar: Option<String>,
    #[validate(phone(message = "please use a valid phone number"))]
    pub phone_number: Option<String>,
    #[serde(skip_serializing)]
    #[validate(
        required,
        length(
            min = 8,
            message = "password may not be less than 8 characters in length"
        )
    )]
    pub password: Option<String>,
}
