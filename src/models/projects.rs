use crate::shared::api_response::EnumerateFields;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
// use validator::Validate;

/// the project model
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ProjectsModel {
    ///the project unique identifier
    pub id: Uuid,
    /// the project name
    pub name: String,
    ///the project description
    pub description: String,
    ///the technologies used, stored as array of string
    pub technologies_used: Vec<String>,
    ///the date the project was added to the database
    pub date_added: String, //TODO: change to dateTime
    ///the project repository url
    pub repo_url: String,
    ///the url of the deployed application if any
    pub live_url: String,
}

///for working with input and output
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInformation {
    /// the project name
    pub name: String,
    ///the project description
    pub description: String,
    ///the technologies used, stored as array of string
    pub technologies_used: Vec<String>,
    ///the date the project was added to the database
    pub date_added: String, //TODO: change to dateTime
    ///the project repository url
    pub repo_url: String,
    ///the url of the deployed application if any
    pub live_url: String,
}

///implement enumerate fields for the project schema
impl EnumerateFields for ProjectInformation {
    /* return a key value pair of the the entries
     * to avoid borrow checker error and possible error from dereferencing,
     * clone the values of the struct
     */
    fn collect_as_strings(&self) -> std::collections::HashMap<String, String> {
        HashMap::from([
            (String::from("projectName"), self.name.clone()),
            (String::from("projectDescription"), self.description.clone()),
            (String::from("repoUrl"), self.repo_url.clone()),
            /* (
                String::from("technologiesUsed"),
                self.technologies_used.clone(),
            ), */
        ])
    }
}
