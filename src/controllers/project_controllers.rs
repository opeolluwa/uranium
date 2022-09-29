use axum::response::IntoResponse;

///create new project
/// accept the following data
/// - projectName  a unique name for the project
/// - projectDescription - the project desription
/// - repoUrl - the project repository
///
pub async fn add_project() -> impl IntoResponse {}

///edit project
/// accept the project id
/// find the project
/// effect edits
/// return updated project object
pub async fn edit_project() -> impl IntoResponse {}

///get all projects
/// retrieve all project with pagination
pub async fn get_all_projects() -> impl IntoResponse {}
