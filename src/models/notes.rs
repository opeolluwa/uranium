// use crate::shared::api_response::EnumerateFields;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
// use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;
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
    pub content: String,
    /// the user_id of the note creator, do ne destructure it when converting this struct to json
    // #[serde(skip_serializing)]
    pub user_id: Option<Uuid>,
    /// the date the note entry was made
    pub date_added: Option<NaiveDateTime>,
    /// the lase date the note was updated
    pub last_updated: Option<NaiveDateTime>,
    /// the note category
    pub category: Option<String>,
}

///for working with input and output
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NotesInformation {
    /// the todo title
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub title: Option<String>,
    ///the note description
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub content: Option<String>,
    /// the entry category
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub category: Option<String>,
}

//implemenmt defaul field for notes
impl Default for NotesInformation {
    fn default() -> Self {
        Self {
            title: Some(String::from("")),
            content: Some(String::from("")),
            category: Some(String::from("")),
        }
    }
}

// ///implement enumerate fields for the note schema
// /// return a key value pair of the the entries
// /// to avoid borrow checker error and possible error from dereferencing,
// /// clone the values of the struct
// impl EnumerateFields for NotesInformation {
//     /* return a key value pair of the the entries
//      * to avoid borrow checker error and possible error from dereferencing,
//      * clone the values of the struct
//      */
//     fn collect_as_strings(&self) -> std::collections::HashMap<String, String> {
//         HashMap::from([
//             (String::from("noteName"), self.title.clone()),
//             // (String::from("noteDescription"), self.content.clone()),
//         ])
//     }
// }
