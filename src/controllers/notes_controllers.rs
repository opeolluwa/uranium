use crate::models::notes::{NotesInformation, NotesModel};
use crate::shared::api_response::{
    ApiErrorResponse, ApiSuccessResponse, Pagination, ValidatedRequest,
};
use crate::shared::jwt_schema::JwtClaims;
use axum::extract::Query;
use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

///create new notes
/// accept the following data
/// - notesName  a unique name for the notes
/// - notesDescription - the notes description
/// - repoUrl - the notes repository
pub async fn add_notes(
    authenticated_user: JwtClaims,
    ValidatedRequest(payload): ValidatedRequest<NotesInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    /*
    save the note
     * generate a UUID and hash the user password,
     * go on to save the hashed password along side other details
     * cat any error along the way
     */
    let notes_id = Uuid::new_v4();
    let new_notes =  sqlx::query_as::<_, NotesModel>(
        "INSERT INTO note_entries (id, title , content, category, user_id ) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO NOTHING RETURNING *",
    )
    .bind(notes_id)
    .bind(payload.title.unwrap_or_default())
    .bind(payload.content.unwrap_or_default())
    .bind(payload.category.unwrap_or_default())
    .bind(sqlx::types::Uuid::parse_str(&authenticated_user.id).unwrap())
    .fetch_one(&database).await;

    //handle error
    match new_notes {
        Ok(note) => {
            //build the response body
            let response_body: ApiSuccessResponse<Value> = ApiSuccessResponse {
                success: true,
                message: String::from("notes successfully added "),
                data: Some(json!({ "note": note })),
            };
            //send the response back to the clien
            Ok((StatusCode::CREATED, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::ServerError {
            message: error_message.to_string(),
        }),
    }
}

///edit notes
/// accept the notes id as route parameter
/// find the notes
/// effect edits
/// return updated notes object
pub async fn edit_note(
    authenticated_user: JwtClaims,
    Path(notes_id): Path<Uuid>,
    ValidatedRequest(payload): ValidatedRequest<NotesInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    //fetch the notes from the database  using the notes id
    let fetched_note = sqlx::query_as::<_, NotesModel>(
        "SELECT * FROM note_entries WHERE id = $1 AND user_id = $2",
    )
    .bind(notes_id)
    .bind(sqlx::types::Uuid::parse_str(&authenticated_user.id).unwrap())
    .fetch_one(&database)
    .await;

    //check if there is a match from the database
    if fetched_note.ok().is_none() {
        return Err(ApiErrorResponse::NotFound {
            message: String::from("note not found"),
        });
    }

    // update the note here then send the response
    let updated_note = sqlx::query_as::<_, NotesModel>(
            "UPDATE note_entries SET title = COALESCE($1, title), content = COALESCE($2, content), category = COALESCE($3, category), last_updated = NOW() WHERE id = $4 AND user_id = $5 RETURNING *",
        )
       .bind(payload.title.unwrap_or_default())
    .bind(payload.content.unwrap_or_default())
    .bind(payload.category.unwrap_or_default())
        .bind(notes_id)
        .bind(sqlx::types::Uuid::parse_str(&authenticated_user.id).unwrap())
        .fetch_one(&database)
        .await;

    //handle errors
    match updated_note {
        Ok(note) => {
            //build the response body body
            let response_body: ApiSuccessResponse<Value> = ApiSuccessResponse {
                success: true,
                message: "notes successfully updated".to_string(),
                data: Some(json!({ "note": note })),
            };
            //return the response with 200 status code
            Ok((StatusCode::OK, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            message: error_message.to_string(),
        }),
    }
}

///get one notes
/// collect the notes id from the client
/// search the database for the notes
/// return success and response or 404 error
pub async fn get_notes_by_id(
    authenticated_user: JwtClaims,
    Path(note_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ApiErrorResponse> {
    //fetch the notes from the database  using the notes id
    let fetched_note = sqlx::query_as::<_, NotesModel>(
        "SELECT * FROM note_entries WHERE id = $1 AND user_id = $2",
    )
    .bind(note_id)
    .bind(sqlx::types::Uuid::parse_str(&authenticated_user.id).unwrap())
    .fetch_one(&database)
    .await;

    //handle errors
    match fetched_note {
        Ok(note) => {
            //build the notes body
            let response_body: ApiSuccessResponse<Value> = ApiSuccessResponse {
                success: true,
                message: String::from("notes successfully retrieved"),
                data: Some(json!({ "note": note })),
            };
            //return the response with 200 status code
            Ok((StatusCode::OK, Json(response_body)))
        }
        Err(_) => Err(ApiErrorResponse::NotFound {
            message: String::from("note could not be found"),
        }),
    }
}

///get all notes
/// retrieve all notes with pagination
pub async fn get_all_notes(
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
    let limit = (current_page - 1) * no_of_rows;

    //fetch all notes  by pagination
    let fetched_notes = sqlx::query_as::<_, NotesModel>(
        "SELECT * FROM note_entries WHERE user_id = $1 ORDER BY date_added DESC LIMIT $2 OFFSET $3",
    )
    .bind(sqlx::types::Uuid::parse_str(&authenticated_user.id).unwrap())
    .bind(no_of_rows)
    .bind(limit)
    .fetch_all(&database)
    .await;

    match fetched_notes {
        Ok(note_array) => {
            //build the Todo body
            let response_body: ApiSuccessResponse<Value> = ApiSuccessResponse {
                success: true,
                message: String::from("notes successfully fetched"),
                data: Some(json!({
                "notes": note_array,
                "currentPage" : &pagination.page.to_string(),
                "noOfRows":&pagination.no_of_rows.to_string()
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

///delete note
/// accept the notes id as route parameter
/// find the notes
/// effect edits
/// return updated notes object
pub async fn delete_note(
    authenticated_user: JwtClaims,
    Path(notes_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<()>>), ApiErrorResponse> {
    //fetch the notes from the database  using the notes id
    let fetched_note = sqlx::query_as::<_, NotesModel>(
        "SELECT * FROM note_entries WHERE id = $1 AND user_id = $2",
    )
    .bind(notes_id)
    .bind(sqlx::types::Uuid::parse_str(&authenticated_user.id).unwrap())
    .fetch_one(&database)
    .await;

    //check if there is a match from the database
    if fetched_note.ok().is_none() {
        return Err(ApiErrorResponse::NotFound {
            message: String::from("note not found"),
        });
    }

    // update the note here then send the response
    let _delete_note =
        sqlx::query_as::<_, NotesModel>("DELETE FROM note_entries WHERE id = $1 AND user_id = $2")
            .bind(notes_id)
            .bind(sqlx::types::Uuid::parse_str(&authenticated_user.id).unwrap())
            .fetch_one(&database)
            .await;

    //handle errors
    //build the response body body
    let response_body: ApiSuccessResponse<()> = ApiSuccessResponse {
        success: true,
        message: "notes successfully deleted".to_string(),
        data: Some(()),
    };
    //return the response with 200 status code
    Ok((StatusCode::OK, Json(response_body)))
}
