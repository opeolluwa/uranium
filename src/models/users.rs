use serde::{Deserialize, Serialize};


/// define the user data structure 
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformation {
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub username: String,
}
