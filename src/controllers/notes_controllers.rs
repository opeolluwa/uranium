use crate::models::notes::{NotesInformation, NotesModel};
use crate::shared::api_response::{ApiErrorResponse, ApiSuccessResponse, EnumerateFields};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::PgPool;
use uuid::Uuid;

///create new notes
/// accept the following data
/// - notesName  a unique name for the notes
/// - notesDescription - the notes description
/// - repoUrl - the notes repository
pub async fn add_notes(
    Json(payload): Json<NotesInformation>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<NotesModel>>), ApiErrorResponse> {
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
    let NotesInformation {
        // the notes name
        name: notes_name,
        //the notes description
        description: notes_description,
        //the technologies used, stored as array of string
        technologies_used,
        //the date the notes was added to the database
        // date_added,
        //the notes repository url
        repo_url,
        //the url of the deployed application if any
        live_url,
    } = &payload;

    // save the new notes
    /*
     * generate a UUID and hash the user password,
     * go on to save the hashed password along side other details
     * cat any error along the way
     */
    let notes_id = Uuid::new_v4();
    let new_notes =  sqlx::query_as::<_, NotesModel>(
        "INSERT INTO notes_information (id, name, description, technologies_used, repo_url, live_url) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (name) DO NOTHING RETURNING *",
    )
    .bind(Some(notes_id))
    .bind(Some(notes_name))
    // .bind(Some(date_added))
    .bind(Some(notes_description))
    .bind(Some(technologies_used))
    .bind(Some(repo_url))
    .bind(Some(live_url))
    .fetch_one(&database).await;

    //handle error
    match new_notes {
        Ok(notes) => {
            //build the response body
            let response_body: ApiSuccessResponse<NotesModel> = ApiSuccessResponse {
                success: true,
                message: "notes successfully added ".to_string(),
                data: Some(notes),
            };
            //send the response
            Ok((StatusCode::CREATED, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::ConflictError {
            error: vec![
                error_message.to_string(),
                "data most likely exists".to_string(),
            ]
            .join(" because "),
        }),
    }
}

///edit notes
/// accept the notes id as route parameter
/// find the notes
/// effect edits
/// return updated notes object
pub async fn edit_notes(
    Path(notes_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<NotesModel>>), ApiErrorResponse> {
    //fetch the notes from the database  using the notes id
    let fetched_notes =
        sqlx::query_as::<_, NotesModel>("SELECT * FROM notes_information WHERE id = $1")
            .bind(notes_id)
            .fetch_one(&database)
            .await;

    //handle errors
    match fetched_notes {
        Ok(notes) => {
            //build the notes body
            let response_body: ApiSuccessResponse<NotesModel> = ApiSuccessResponse {
                success: true,
                message: "notes successfully retrieved".to_string(),
                data: Some(notes),
            };
            //return the response with 200 status code
            Ok((StatusCode::OK, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            error: error_message.to_string(),
        }),
    }
}

///get one notes
/// collect the notes id from the client
/// search the database for the notes
/// return success and response or 404 error
pub async fn get_notes_by_id(
    Path(note_id): Path<Uuid>,
    Extension(database): Extension<PgPool>,
) -> Result<(StatusCode, Json<ApiSuccessResponse<NotesModel>>), ApiErrorResponse> {
    //fetch the notes from the database  using the notes id
    let fetched_notes =
        sqlx::query_as::<_, NotesModel>("SELECT * FROM notes_information WHERE id = $1")
            .bind(note_id)
            .fetch_one(&database)
            .await;

    //handle errors
    match fetched_notes {
        Ok(notes) => {
            //build the notes body
            let response_body: ApiSuccessResponse<NotesModel> = ApiSuccessResponse {
                success: true,
                message: "notes successfully retrieved".to_string(),
                data: Some(notes),
            };
            //return the response with 200 status code
            Ok((StatusCode::OK, Json(response_body)))
        }
        Err(error_message) => Err(ApiErrorResponse::NotFound {
            error: error_message.to_string(),
        }),
    }
}

///get all notes
/// retrieve all notes with pagination
pub async fn get_all_notes(Extension(database): Extension<PgPool>) -> impl IntoResponse {
    //fetch all notes ...
    //TODO: implement pagination logic
    let _fetched_notes =
        sqlx::query_as::<_, NotesModel>("SELECT * FROM notes_information").fetch(&database);
    todo!()
}