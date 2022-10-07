use crate::shared::api_response::EnumerateFields;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::collections::HashMap;

use super::emails::EmailModel;

/// define the user data structure that shall serve as the basis of serial
/// implement debug, serialize, deserializing and #[derive(sqlx::FromRow
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserModel {
    ///the user uniques identifier
    pub id: Uuid,
    ///the user fullname
    pub fullname: String,
    ///the user email
    pub email: String,
    ///the user password, don't return the password when fetching the user data
    #[serde(skip_serializing)]
    pub password: String,
    /// the user username
    pub username: String,
}

///user authorization information
/// to be used for making login in requests
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserAuthCredentials {
    //the user email
    pub email: String,
    ///the user password
    pub password: String,
}

///the user information is derived from the user model
/// it shall be responsible for providing the user information such as in JWT encryption
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserInformation {
    /// the user fullname
    pub fullname: String,
    /// the user email
    pub email: String,
    /// the user password
    #[serde(skip_serializing)]
    pub password: String,
    /// the user unique username
    pub username: String,
}

///return the UserInformation as an array of
impl EnumerateFields for UserInformation {
    fn collect_as_strings(&self) -> HashMap<String, String> {
        /* return a key value pair of the the entries
         * to avoid borrow checker error and possible error from dereferencing,
         * clone the values of the struct
         */
        HashMap::from([
            (String::from("fullname"), self.fullname.clone()),
            (String::from("username"), self.username.clone()),
            (String::from("password"), self.password.clone()),
            (String::from("email"), self.email.clone()),
        ])
    }
}

///return the UserInformation as an array of
impl EnumerateFields for UserAuthCredentials {
    fn collect_as_strings(&self) -> HashMap<String, String> {
        /* return a key value pair of the the entries
         * to avoid borrow checker error and possible error from dereferencing,
         * clone the values of the struct
         */
        HashMap::from([
            (String::from("password"), self.password.clone()),
            (String::from("email"), self.email.clone()),
        ])
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
