use super::emails::EmailModel;
use crate::utils::api_response::EnumerateFields;
use crate::utils::sql_query_builder::{Create, FindByPk};
use async_trait::async_trait;
use bcrypt::DEFAULT_COST;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use validator::Validate;

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
    Unspecified,
}
/// define the user data structure that shall serve as the basis of serial
/// implement debug, serialize, deserializing and #[derive(sqlx::FromRow to make the struct operable
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserModel {
    pub id: Uuid,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub middlename: Option<String>,
    pub fullname: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub account_status: Option<AccountStatus>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<UserGender>,
    pub avatar: Option<String>,
    pub phone_number: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub otp_id: Option<Uuid>,
    pub last_available_at: Option<NaiveDateTime>,
}

///the user information is derived from the user model
/// it shall be responsible for providing the user information such as in JWT encryption
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserInformation {
    // pub id: Uuid,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub middlename: Option<String>,
    pub fullname: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub account_status: Option<AccountStatus>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<UserGender>,
    pub avatar: Option<String>,
    pub phone_number: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub last_available_at: Option<NaiveDateTime>,
}

/// associated functions and methods
impl UserModel {
    /// has a user password
    pub fn hash_pswd(password: Option<String>) -> String {
        let password = password.unwrap();
        bcrypt::hash(password.trim(), DEFAULT_COST).unwrap()
    }
    /// verify hashed password
    pub fn verify_pswd_hash(&self, raw_password: &str) -> bool {
        let stored_password = self.password.as_ref().unwrap();
        bcrypt::verify(raw_password.to_string(), &stored_password)
            .ok()
            .is_some()
    }
}

/// implement query builder traits for UserModel
#[async_trait]
impl Create for UserModel {
    type Entity = UserModel;
    type Attributes = UserInformation;
    // type UpdatedAttribute = dyn Any;
    /// save a new record in the database
    async fn create(
        fields: Self::Attributes,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error> {
        let Self::Attributes {
            firstname,
            lastname,
            middlename,
            fullname,
            username,
            email,
            date_of_birth,
            gender,
            avatar,
            phone_number,
            password,
            ..
        } = fields;
        let sql_query = r#"
INSERT INTO
    user_information (
        id,gender,firstname,lastname,middlename,
        fullname,username,email, date_of_birth,avatar, phone_number,
        password
    ) VALUES
    ( $1, $2, NUllIF($3, ''), NUllIF($4, ''), NUllIF($5, ''),
        NUllIF($6, ''),NUllIF($7, ''), NUllIF($8, null),
        NUllIF($9, null), NUllIF($10, ''), NUllIF($11, ''), NULLIF($12, '')
    ) ON CONFLICT (email) DO NOTHING RETURNING *
    "#;
        let id = Uuid::new_v4();
        let hashed_password = UserModel::hash_pswd(password);
        let new_user = sqlx::query_as::<_, UserModel>(sql_query)
            .bind(id)
            .bind(gender.unwrap_or_default())
            .bind(firstname.unwrap_or_default())
            .bind(lastname.unwrap_or_default())
            .bind(middlename.unwrap_or_default())
            .bind(fullname.unwrap_or_default())
            .bind(username.unwrap_or_default())
            .bind(email.unwrap_or_default().trim())
            .bind(date_of_birth.unwrap_or_default())
            .bind(avatar.unwrap_or_default())
            .bind(phone_number.unwrap_or_default())
            .bind(hashed_password)
            .fetch_one(db_connection)
            .await;
        new_user
    }
}

///implement find by PK for user Model
#[async_trait]
impl FindByPk for UserModel {
    type Entity = UserModel;
    type Attributes = UserInformation;
    /// find user by id
    async fn find_by_pk(
        id: &str,
        db_connection: &Pool<Postgres>,
    ) -> Result<Self::Entity, sqlx::Error> {
        sqlx::query_as::<_, UserModel>("SELECT * FROM user_information WHERE id = $1")
            .bind(sqlx::types::Uuid::parse_str(id).unwrap())
            .fetch_one(db_connection)
            .await
    }
}

// #[async_trait]
// /// impl Update Entity of user model
// impl UpdateEntity for UserModel {
//     type Entity = UserModel;
//     //TODO: make the update filed take an array of generic hashmaps, representing the updates
//     async fn update(
//         &self,
//         fields: Vec<std::collections::HashMap<String, String>>,
//         db_connection: &Pool<Postgres>,
//     ) -> Result<Self::Entity, sqlx::Error> {
//         let key = "";
//         let value = "";

//         sqlx::query_as::<_, UserModel>("UPDATE user_information SET $1 = $2 WHERE id = $3")
//             .bind(&key)
//             .bind(&value)
//             .fetch_one(db_connection)
//             .await
//     }
// }

///user authorization information
/// to be used for making login and sign up requests
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Validate)]
pub struct UserAuthCredentials {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    /// the user fullname set to optional to allow use of struct for bothe login and sign up
    pub fullname: Option<String>,
}

/// implement default value for user gender
impl Default for UserGender {
    fn default() -> Self {
        Self::Unspecified
    }
}

// impl Default for UserInformation {
//     fn default() -> Self {
//         None
//     }
// }
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
