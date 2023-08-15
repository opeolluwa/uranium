use serde::{Deserialize, Serialize};
use validator::Validate;

/// login
pub struct Login {
    username: String,
    password: String,
}

/// sign up
pub struct SignUp;

/// reset password
pub struct ResetPassword;

///the user information is derived from the user model
/// it shall be responsible for providing the user information such as in JWT encryption
#[derive(Debug, Serialize, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserInformation {
    // pub id: Uuid,
    // #[validate(required, length(min = 1))]
    pub firstname: Option<String>,
    // #[validate(required, length(min = 1))]
    pub lastname: Option<String>,
    // #[validate(required, length(min = 1))]
    pub middlename: Option<String>,
    // #[validate(required, length(min = 1))]
    pub fullname: Option<String>,
    // #[validate(required, length(min = 1))]
    pub username: Option<String>,
    // #[validate(required, email(message = "please use a valid email"))]
    pub email: Option<String>,
    // pub account_status: Option<AccountStatus>,
    // pub date_of_birth: Option<NaiveDate>,
    // pub gender: Option<UserGender>,
    // #[validate(url(message = "invalid URL detected"))]
    pub avatar: Option<String>,
    // #[validate(phone(message = "please use a valid phone number"))]
    pub phone_number: Option<String>,
    // #[serde(skip_serializing)]
    /*  #[validate(
        required,
        length(
            min = 8,
            message = "password may not be less than 8 characters in length"
        )
    )] */
    // pub password: Option<String>,
    // pub created_at: Option<NaiveDateTime>,
    // pub updated_at: Option<NaiveDateTime>,
    // pub last_available_at: Option<NaiveDateTime>,
}
