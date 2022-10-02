use crate::{
    models::projects::{ProjectInformation, ProjectsModel},
    shared::api_response::{ApiErrorResponse, ApiSuccessResponse, EnumerateFields},
};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::PgPool;
use uuid::Uuid;

///create new project
/// accept the following data
/// - projectName  a unique name for the project
/// - projectDescription - the project description
/// - repoUrl - the project repository
///
pub async fn add_project(
    Json(payload): Json<ProjectInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<ProjectsModel>>), ApiErrorResponse> {
    //check through the fields to see that no field was badly formatted
    let entries = &payload.collect_as_strings();
    let mut bad_request_errors: Vec<String> = Vec::new();
    for (key, value) in entries {
        if value.is_empty() {
            let error = format!("{key} is empty");
            bad_request_errors.push(error);
        }
    }

    // destructure the payload
    let ProjectInformation {
        // the project name
        name: project_name,
        //the project description
        description: project_description,
        //the technologies used, stored as array of string
        technologies_used,
        //the date the project was added to the database
        date_added,
        //the project repository url
        repo_url,
        //the url of the deployed application if any
        live_url,
    } = &payload;

    // save the new project
    /*
     * generate a UUID and hash the user password,
     * go on to save the hashed password along side other details
     * cat any error along the way
     */
    let project_id = Uuid::new_v4();
    let new_project =  sqlx::query_as::<_, ProjectsModel>(
        "INSERT INTO project_information (id, name, description, date_added, technologies_used, repo_url, live_url) VALUES ($1, $2, $3, $4, $5, $, $7) ON CONFLICT (name) DO NOTHING RETURNING *",
    )
    .bind(Some(project_id))
    .bind(Some(project_name))
    .bind(Some(date_added))
    .bind(Some(project_description))
    .bind(Some(technologies_used))
    .bind(Some(repo_url))
    .bind(Some(live_url))
    .fetch_one(&database).await;

    //handle error
    match new_project {
        Ok(project) => {
            //build the response body
            let response_body: ApiSuccessResponse<ProjectsModel> = ApiSuccessResponse {
                success: true,
                message: "project successfully added ".to_string(),
                data: Some(project),
            };
            //send the response
            Ok((StatusCode::CREATED, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::ServerError {
            error: error_message.to_string(),
        }),
    }
}

///edit project
/// accept the project id
/// find the project
/// effect edits
/// return updated project object
pub async fn edit_project() -> impl IntoResponse {}

///get all projects
/// retrieve all project with pagination
pub async fn get_all_projects(
    Extension(database): Extension<PgPool>,
) -> impl IntoResponse {
    //fetch all projects ...
    //TODO: implement pagination logic
     let fetched_projects = sqlx::query_as::<_, ProjectsModel>("SELECT * FROM project_information")
        .fetch(&database);
 
    todo!()
}
