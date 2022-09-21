use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::collections::HashMap;

/// define the user data structure
/// implement debug, serialize, deserializing and #[derive(sqlx::FromRow
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserInformation {
    pub id: Uuid,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub username: String,
}

///user authorization information
/// to be used for making access retrieve user information
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserAuthCredentials {
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub username: String,
}

///  a trait to return the field of the structs as an array of strings
///  the implementation on user information will return the user is, firstname, username ...
/// on the user authentication struct, the implementation will return the user email and password

pub trait EnumerateFields {
    fn collect_as_strings(&self) -> HashMap<String, String>;
}

///return the UserInformation as an array of
impl EnumerateFields for UserAuthCredentials {
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
