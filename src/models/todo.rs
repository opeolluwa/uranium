use crate::shared::api_response::EnumerateFields;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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
}

///for working with input and output
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoInformation {
    pub title: String,
    ///the note description
    pub description: String,
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
