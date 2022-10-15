use crate::shared::api_response::EnumerateFields;
// use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// the note model
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TodoModel {
    ///the todo unique identifier
    pub id: Uuid,
    /// the todo title
    pub title: String,
    ///the todo details
    pub description: String,
    /// the user_id of the todo creator, do ne destructure it when converting this struct to json
    #[serde(skip_serializing)]
    pub fk_user_id: Uuid,
    /// the todo due date
    // pub due_date: chrono::DateTime<chrono::Utc>,
    /// the todo priority
    pub priority: String,
}

///for working with input and output
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TodoInformation {
    /// the todo title
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub title: String,
    ///the todo description
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub description: String,
    /// the todo due date
    // #[validate(length(min = 1, message = "Can not be empty"))]
    // pub due_date: chrono::DateTime<chrono::Utc>,
    /// the todo priority
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub priority: String,
}

///implement default for TodoInformation
/// this will set the fields to empty strings
/// then the handler (controller) will check if values are empty strings or not,
/// if empty string, the handlers will throw off bad request error

impl Default for TodoInformation {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            description: "".to_string(),
            // due_date: Utc::now(),
            priority: "unset".to_string(),
        }
    }
}
///implement enumerate fields for the note schema
impl EnumerateFields for TodoInformation {
    /* return a key value pair of the the entries
     * to avoid borrow checker error and possible error from dereferencing,
     * clone the values of the struct
     */
    fn collect_as_strings(&self) -> std::collections::HashMap<String, String> {
        HashMap::from([
            (String::from("todoTitle"), self.title.clone()),
            (String::from("todoDetails"), self.description.clone()),
        ])
    }
}
