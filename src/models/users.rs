use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::collections::HashMap;

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
    ///the user password
    pub password: String,
    /// the user username
    pub username: String,
}

///user authorization information
/// to be used for making login in requests
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserAuthCredentials {
    ///the user email
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
    pub password: String,
    /// the user unique username
    pub username: String,
}

///  a trait to return the field of the structs as an array of strings
///  the implementation on user information will return the user is, firstname, username ...
/// on the user authentication struct, the implementation will return the user email and password

pub trait EnumerateFields {
    fn collect_as_strings(&self) -> HashMap<String, String>;
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
