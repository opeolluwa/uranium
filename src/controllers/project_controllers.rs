use crate::utils::api_response::{ApiErrorResponse, ApiSuccessResponse};
use crate::utils::jwt::JwtClaims;
use crate::models::projects::{ProjectInformation, ProjectsModel};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::PgPool;
use uuid::Uuid;

///create new project
/// accept the following data
/// - projectName  a unique name for the project
/// - projectDescription - the project description
/// - repoUrl - the project repository
///
pub async fn add_project(
    _claims: JwtClaims,
    Json(payload): Json<ProjectInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<ProjectsModel>>), ApiErrorResponse> {
    //check through the fields to see that no field was badly formatted

    // destructure the payload
    let ProjectInformation {
        // the project name
        name: project_name,
        //the project description
        description: project_description,
        //the technologies used, stored as array of string
        technologies_used,
        //the date the project was added to the database
        // date_added,
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
        "INSERT INTO project_information (id, name, description, technologies_used, repo_url, live_url) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (name) DO NOTHING RETURNING *",
    )
    .bind(Some(project_id))
    .bind(Some(project_name))
    // .bind(Some(date_added))
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
        Err(error_message) => Err(ApiErrorResponse::ConflictError {
            message: vec![
                error_message.to_string(),
                "data most likely exists".to_string(),
            ]
            .join(" because "),
        }),
    }
}

///edit project
/// accept the project id as route parameter
/// find the project
/// effect edits
/// return updated project object
pub async fn edit_project(
    _claims: JwtClaims,
    Path(project_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<ProjectsModel>>), ApiErrorResponse> {
    //fetch the project from the database  using the project id
    let fetched_project =
        sqlx::query_as::<_, ProjectsModel>("SELECT * FROM project_information WHERE id = $1")
            .bind(project_id)
            .fetch_one(&database)
            .await;

    //handle errors
    match fetched_project {
        Ok(project) => {
            //build the project body
            let response_body: ApiSuccessResponse<ProjectsModel> = ApiSuccessResponse {
                success: true,
                message: "Project successfully retrieved".to_string(),
                data: Some(project),
            };
            //return the response with 200 status code
            Ok((StatusCode::OK, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            message: error_message.to_string(),
        }),
    }
}

///get one project
/// collect the project id from the client
/// search the database for the project
/// return success and response or 404 error
pub async fn get_project_by_id(
    _claims: JwtClaims,
    Path(project_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<ProjectsModel>>), ApiErrorResponse> {
    //fetch the project from the database  using the project id
    let fetched_project =
        sqlx::query_as::<_, ProjectsModel>("SELECT * FROM project_information WHERE id = $1")
            .bind(project_id)
            .fetch_one(&database)
            .await;

    //handle errors
    match fetched_project {
        Ok(project) => {
            //build the project body
            let response_body: ApiSuccessResponse<ProjectsModel> = ApiSuccessResponse {
                success: true,
                message: "Project successfully retrieved".to_string(),
                data: Some(project),
            };
            //return the response with 200 status code
            Ok((StatusCode::OK, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            message: error_message.to_string(),
        }),
    }
}

///get all projects
/// retrieve all project with pagination
pub async fn get_all_projects(
    _claims: JwtClaims,
    Extension(database): Extension<PgPool>,
) -> impl IntoResponse {
    //fetch all projects ...
    //TODO: implement pagination logic
    let _fetched_projects =
        sqlx::query_as::<_, ProjectsModel>("SELECT * FROM project_information").fetch(&database);

    todo!()
}
