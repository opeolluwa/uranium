use crate::models::todo::{TodoInformation, TodoModel};
use crate::shared::{
    api_response::{ApiErrorResponse, ApiSuccessResponse, EnumerateFields},
    jwt_schema::JwtClaims,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

///create new Todo
/// accept the following data
/// - TodoName  a unique name for the Todo
/// - TodoDescription - the Todo description
/// - repoUrl - the Todo repository
pub async fn add_todo(
    authenticated_user: JwtClaims,
    Json(payload): Json<TodoInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    //check through the fields to see that no field was badly formatted
    let entries = &payload.collect_as_strings();
    let mut bad_request_errors: Vec<String> = Vec::new();
    for (key, value) in entries {
        if value.is_empty() {
            let error = format!("{key} is empty");
            bad_request_errors.push(error);
        }
    }

    // save the new Todo
    /*
     * generate a UUID and hash the user password,
     * go on to save the hashed password along side other details
     * cat any error along the way
     */
    let todo_id = Uuid::new_v4();
    let new_todo =  sqlx::query_as::<_, TodoModel>(
        "INSERT INTO todo (id, title, description, fk_user_id) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO NOTHING RETURNING *",
    )
    .bind(todo_id)
    .bind(payload.title)
    .bind(payload.description)
    .bind(authenticated_user.id)
    .fetch_one(&database).await;

    //handle error
    match new_todo {
        Ok(todo) => {
            //build the response body
            let response_body: ApiSuccessResponse<Value> = ApiSuccessResponse {
                success: true,
                message: "Todo successfully added ".to_string(),
                data: Some(json!({
                    "todo":TodoModel{..todo}
                })),
            };
            //send the response
            Ok((StatusCode::CREATED, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::ServerError {
            error: error_message.to_string(),
        }),
    }
}

///edit Todo
/// accept the Todo id as route parameter
/// find the Todo
/// effect edits
/// return updated Todo object
pub async fn edit_todo(
    _claims: JwtClaims,
    Path(Todo_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<TodoModel>>), ApiErrorResponse> {
    //fetch the Todo from the database  using the Todo id
    let fetched_Todo = sqlx::query_as::<_, TodoModel>("SELECT * FROM Todo WHERE id = $1")
        .bind(Todo_id)
        .fetch_one(&database)
        .await;

    //handle errors
    match fetched_Todo {
        Ok(Todo) => {
            //build the Todo body
            let response_body: ApiSuccessResponse<TodoModel> = ApiSuccessResponse {
                success: true,
                message: "Todo successfully retrieved".to_string(),
                data: Some(Todo),
            };
            //return the response with 200 status code
            Ok((StatusCode::OK, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            error: error_message.to_string(),
        }),
    }
}

///get one Todo
/// collect the Todo id from the client
/// search the database for the Todo
/// return success and response or 404 error
pub async fn get_todo_by_id(
    _claims: JwtClaims,
    Path(note_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<TodoModel>>), ApiErrorResponse> {
    //fetch the Todo from the database  using the Todo id
    let fetched_Todo = sqlx::query_as::<_, TodoModel>("SELECT * FROM Todo WHERE id = $1")
        .bind(note_id)
        .fetch_one(&database)
        .await;

    //handle errors
    match fetched_Todo {
        Ok(Todo) => {
            //build the Todo body
            let response_body: ApiSuccessResponse<TodoModel> = ApiSuccessResponse {
                success: true,
                message: "Todo successfully retrieved".to_string(),
                data: Some(Todo),
            };
            //return the response with 200 status code
            Ok((StatusCode::OK, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            error: error_message.to_string(),
        }),
    }
}

///get all Todo
/// retrieve all Todo with pagination
pub async fn get_all_todo(
    _claims: JwtClaims,
    Extension(database): Extension<PgPool>,
) -> impl IntoResponse {
    //fetch all Todo ...
    //TODO: implement pagination logic
    let _fetched_Todo = sqlx::query_as::<_, TodoModel>("SELECT * FROM Todo").fetch(&database);
    todo!()
}
