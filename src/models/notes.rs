use crate::shared::api_response::EnumerateFields;
// use sqlx::types::time::Date;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
// use validator::Validate;

/// the note model
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct NotesModel {
    ///the note unique identifier
    pub id: Uuid,
    /// the note title
    pub title: String,
    ///the note description
    pub description: String,
    /// the user_id of the note creator, do ne destructure it when converting this struct to json
    #[serde(skip_serializing)]
    pub fk_user_id: Uuid,
}

///for working with input and output
#[derive(Debug, Serialize, Deserialize)]
pub struct NotesInformation {
    /// the todo title
    pub title: String,
    ///the note description
    pub description: String,
}

///implement enumerate fields for the note schema
/// return a key value pair of the the entries
/// to avoid borrow checker error and possible error from dereferencing,
/// clone the values of the struct
impl EnumerateFields for NotesInformation {
    /* return a key value pair of the the entries
     * to avoid borrow checker error and possible error from dereferencing,
     * clone the values of the struct
     */
    fn collect_as_strings(&self) -> std::collections::HashMap<String, String> {
        HashMap::from([
            (String::from("noteName"), self.title.clone()),
            (String::from("noteDescription"), self.description.clone()),
        ])
    }
}
