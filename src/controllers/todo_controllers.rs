use crate::utils::jwt::JwtClaims;
use crate::models::todo::{TodoInformation, TodoModel};
use axum::extract::Query;
use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use sqlx::PgPool;
use crate::utils::api_response::{ApiErrorResponse, ApiSuccessResponse};
use crate::utils::api_response::{Pagination, ValidatedRequest};
use uuid::Uuid;

///create new Todo
/// accept the following data
/// - TodoName  a unique name for the Todo
/// - TodoDescription - the Todo description
/// - repoUrl - the Todo repository
pub async fn add_todo(
    authenticated_user: JwtClaims,
    ValidatedRequest(payload): ValidatedRequest<TodoInformation>,
    // Json(payload): Json<TodoInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    //  if no error save the new Todo
    /*
     * generate a UUID and hash the user password,
     * go on to save the hashed password along side other details
     * cat any error along the way
     */
    let todo_id = Uuid::new_v4();
    let new_todo =  sqlx::query_as::<_, TodoModel>(
        "INSERT INTO todo_list (id, title, description, user_id, priority) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO NOTHING RETURNING *",
    )
    .bind(todo_id)
    .bind(payload.title)
    .bind(payload.description)
    .bind(sqlx::types::Uuid::parse_str(&authenticated_user.id).unwrap())
    .bind(payload.priority)
    // .bind("now()")

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
            message: error_message.to_string(),
        }),
    }
}

///edit Todo
/// accept the Todo id as route parameter
/// find the Todo
/// effect edits
/// return updated Todo object
pub async fn edit_todo(
    authenticated_user: JwtClaims,
    Path(todo_id): Path<Uuid>,
    Json(payload): Json<TodoInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    //fetch the Todo from the database  using the Todo id
    let updated_todo = sqlx::query_as::<_, TodoModel>("UPDATE todo_list SET title = COALESCE($1, title), description = COALESCE($2 , description), last_update = NOW() WHERE user_id = $3 AND id = $4")
        .bind(payload.title)
        .bind(payload.description)
        .bind(sqlx::types::Uuid::parse_str(&authenticated_user.id).unwrap())
        .bind(todo_id)
        .fetch_one(&database)
        .await;

    //handle errors
    match updated_todo {
        Ok(todo) => {
            //build the Todo body
            let response_body: ApiSuccessResponse<Value> = ApiSuccessResponse {
                success: true,
                message: "Todo successfully updated".to_string(),
                data: Some(json!({
                    "todo":TodoModel{..todo}
                })),
            };
            //return the response with 200 status code
            Ok((StatusCode::OK, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            message: error_message.to_string(),
        }),
    }
}

///get one Todo
/// collect the Todo id from the client
/// search the database for the Todo
/// return success and response or 404 error
pub async fn get_todo_by_id(
    authenticated_user: JwtClaims,
    Path(note_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<TodoModel>>), ApiErrorResponse> {
    //fetch the Todo from the database  using the Todo id
    let fetched_todo =
        sqlx::query_as::<_, TodoModel>("SELECT * FROM Todo WHERE id = $1 AND user_id = $2")
            .bind(note_id)
            .bind(sqlx::types::Uuid::parse_str(&authenticated_user.id).unwrap())
            .fetch_one(&database)
            .await;

    //handle errors
    match fetched_todo {
        Ok(todo) => {
            //build the Todo body
            let response_body: ApiSuccessResponse<TodoModel> = ApiSuccessResponse {
                success: true,
                message: "Todo successfully retrieved".to_string(),
                data: Some(todo),
            };
            //return the response with 200 status code
            Ok((StatusCode::OK, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            message: error_message.to_string(),
        }),
    }
}

///get all Todo
/// retrieve all Todo with pagination
/// the response will contain a data object will will contain
///  the current page,
///  number of rows per page
///  a vector of TodoModel which are essentially an array of fetched todo
pub async fn get_all_todo(
    authenticated_user: JwtClaims,
    pagination: Option<Query<Pagination>>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    // try and get the quey params or deflect to default
    // let pagination_params = query_params;
    let Query(pagination) = pagination.unwrap_or_default();
    let Pagination {
        page: current_page,
        no_of_rows,
    } = &pagination;

    // REFINE THIS if the page is 1, don't use offset else do
    // the base query is SELECT * FROM todo_list ORDER BY date_added  DESC LIMIT 2 OFFSET 0 ;
    let query: &str = if current_page > &1i32 {
        "SELECT * FROM todo_list WHERE user_id = $3 ORDER BY date_added DESC LIMIT $1 OFFSET $2 "
    } else {
        "SELECT * FROM todo_list WHERE user_id = $3 ORDER BY date_added DESC LIMIT $1"
    };
    // let current_page = &query_params.page.trim().parse().unwrap();
    //implement pagination logic
    let fetched_todo = sqlx::query_as::<_, TodoModel>(query)
        .bind(no_of_rows)
        .bind(current_page * no_of_rows)
        .bind(sqlx::types::Uuid::parse_str(&authenticated_user.id).unwrap())
        .fetch_all(&database)
        .await;

    // println!("{:#?}", fetched_todo);
    //error handling
    match fetched_todo {
        Ok(todo_array) => {
            //build the Todo body
            let response_body: ApiSuccessResponse<Value> = ApiSuccessResponse {
                success: true,
                message: "Todo successfully updated".to_string(),
                data: Some(json!({
                         "todo": todo_array, "currentPage" : &pagination.page.to_string(),  "noOfRows":&pagination.no_of_rows.to_string()})),
            };
            //return the response with 200 status code
            Ok((StatusCode::OK, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            message: error_message.to_string(),
        }),
    }
}

/// delete todo
/// accept the todo id from an authenticated user,
/// check if the user is the owner of the todo
/// delete the tod and return a respone
pub async fn delete_todo(
    authenticated_user: JwtClaims,
    Path(todo_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<()>>), ApiErrorResponse> {
    //fetch the Todo from the database  using the Todo id
    let fetched_todo = sqlx::query_as::<_, TodoModel>(
        "DELETE FROM todo_list WHERE id = $1 AND user_id = $2 RETURNING *",
    )
    .bind(todo_id)
    .bind(sqlx::types::Uuid::parse_str(&authenticated_user.id).unwrap())
    .fetch_one(&database)
    .await;

    //handle errors
    match fetched_todo {
        Ok(_) => {
            //build the Todo body
            let response_body: ApiSuccessResponse<_> = ApiSuccessResponse {
                success: true,
                message: "Todo successfully deleted".to_string(),
                data: None,
            };
            //return the response with 200 status code
            Ok((StatusCode::OK, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            message: error_message.to_string(),
        }),
    }
}
