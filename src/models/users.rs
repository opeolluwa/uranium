use super::emails::EmailModel;
use crate::shared::api_response::EnumerateFields;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::collections::HashMap;
use validator::Validate;
use sqlx::types::chrono::NaiveDateTime;

/// an enum stating the user current account status
/// the variants are active, inactive, Suspended and Deactivated. The account status is essential especially for access control and authorization
#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[sqlx(type_name = "account_status")] // only for PostgreSQL to match a type definition
#[sqlx(rename_all = "lowercase")]
pub enum AccountStatus {
    Active,
    Inactive,
    Suspended,
    Deactivated,
}

/// an enum stating the user current gender type
#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[sqlx(type_name = "gender")] // only for PostgreSQL to match a type definition
#[sqlx(rename_all = "lowercase")]
pub enum UserGender {
   Male,
   Others,
   Female,
   Unspecified
}
/// define the user data structure that shall serve as the basis of serial
/// implement debug, serialize, deserializing and #[derive(sqlx::FromRow to make the struct operable 
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserModel {
    ///the user uniques identifier
    pub id: Uuid,
    /// the user firstname
    pub firstname: Option<String>,
    /// the user lastname
    pub lastname: Option<String>,
    /// the user middle name
    pub middlename: Option<String>,
    ///the user fullname
    pub fullname: Option<String>,
    /// the user name
    pub username: Option<String>,
    ///the user email
    pub email: Option<String>,
    /// the user account status
    pub account_status: Option<AccountStatus>,
    /// the user date of birthday
    pub date_of_birth: Option<NaiveDateTime>,
    /// the user gender
    pub gender :Option<UserGender>,
    /// the user profile picture URL
    pub avatar: Option<String>,
    /// the String data type is used in storing phone number to allow storing it with country code 
    /// example +44632900, +2342940474
    pub phone_number : Option<String>,
    /// the user password, 
    /// in deserializing the user data,
    ///  don't return the password when fetching the user data
    #[serde(skip_serializing)]
    pub password: Option<String>,
    /// the date the account was created 
    pub created_at :Option<NaiveDateTime>,
    /// the last date the account information was updates 
    pub updated_at : Option<NaiveDateTime>,
    /// the last login  date 
    pub last_available_at : Option<NaiveDateTime>
}



///user authorization information
/// to be used for making login and sign up requests
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Validate)]
pub struct UserAuthCredentials {
    //the user email
    #[validate(email)]
    pub email: String,
    ///the user password
    #[validate(length(min = 8))]
    pub password: String,
    /// the user fullname set to optional to allow use of struct for bothe login and sign up
    pub fullname: Option<String>,
}

///the user information is derived from the user model
/// it shall be responsible for providing the user information such as in JWT encryption
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Validate)]
pub struct UserInformation {
    /// the user email
    pub email: String,
    /// the user password
    #[serde(skip_serializing)]
    pub password: String,
    /// the user unique username
    pub username: Option<String>,
    /// the user fullname
    // #[validate(length(min = 1 "cannot be empty"))]
    pub fullname: Option<String>,
}

impl Default for UserInformation {
    fn default() -> Self {
        Self {
            email: String::from(""),
            password: String::from(""),
            username: Some(String::from("")),
            fullname: Some(String::from("")),
        }
    }
}

/// the user reset password payload structure
/// the payload will implement EnumerateFields to validate the payload
/// it will also derive the rename-all trait of serde to all the use of JavaScript's camel case convection
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetUserPassword {
    pub new_password: String,
    pub confirm_password: String,
}

/// implement Enumerate fields for Reset UserPassword
impl EnumerateFields for ResetUserPassword {
    /* return a key value pair of the the entries
     * to avoid borrow checker error and possible error from dereferencing,
     * clone the values of the struct
     */
    fn collect_as_strings(&self) -> std::collections::HashMap<String, String> {
        HashMap::from([
            (String::from("newPassword"), self.new_password.clone()),
            (
                String::from("confirmPassword"),
                self.confirm_password.clone(),
            ),
        ])
    }
}

/// the user dashboard data
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserAccountInformation {
    profile: UserModel,
    emails: Vec<EmailModel>,
}
