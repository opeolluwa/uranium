use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde::{Deserialize, Serialize};
/// the API response is supposed to be an enum of two variants
/// ApiResponse::Success<D:Data> and ApiResponse::Error<E:Error>
///
/// ApiResponse::Success is a generic datatype that will return data if any
/// ApiResponse::Error is also a generic datatype that returns error with an optional error detail if any
///
/// however the two types has been merged into one type  ApiResponse<Data, Error>
///Api Response definition
///
/// #Example
/// use crate::ApiResponse
/// let success :ApiResponse<Data, _>
/// let error : ApiResponse<_, Error>
/// let neither_data_nor_error : ApiResponse<_,_>
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<Data, Error> {
    pub success: bool,
    pub message: String,
    pub data: Option<Data>,
    pub error: Option<Error>,
}

///Api success response
/// the api success response returns succes
/// accepts message and data from handle/controller
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiSuccessResponse<Data> {
    pub success: bool,
    pub message: String,
    pub data: Option<Data>,
}

///ApiEzlsAAnVChDQJEZDW9pAq7ks98gjolpfASBHAi8BJ3Y9TeUDHcX9HovV5BzrS4hUKX5tBmB4acfQrrorResponse
/// the error content should be returned as an error of string
// #[allow(dead_code)]
pub enum ApiErrorResponse {
    WrongCredentials { error: String },
    BadRequest { error: String },
    ServerError { error: String },
    ConflictError { error: String },
    InvalidToken { error: String },
    NotFound { error: String },
}

///implement into response trait for api error
impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> Response {
        let (status_code, error_message, error_details) = match self {
            ApiErrorResponse::WrongCredentials { error } => {
                //missing Authorization credentials
                (
                    StatusCode::UNAUTHORIZED,
                    String::from("Wrong or missing authorization credentials"),
                    error.clone(),
                )
            }
            ApiErrorResponse::BadRequest { error } => (
                StatusCode::BAD_REQUEST,
                String::from("Badly formatted or missing credentials"),
                error.clone(),
            ),
            ApiErrorResponse::ServerError { error } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal Server Response"),
                error.clone(),
            ),
            ApiErrorResponse::InvalidToken { error } => (
                StatusCode::BAD_REQUEST,
                String::from("Invalid token or missing authorization token"),
                error.clone(),
            ),
            ApiErrorResponse::ConflictError { error } => (
                StatusCode::CONFLICT,
                String::from("The record you are trying to create already exists"),
                error.clone(),
            ),
            //not found error
            ApiErrorResponse::NotFound { error } => (
                StatusCode::NOT_FOUND,
                String::from("The requested Resource was not found"),
                error.clone(),
            ),
        };
        //build the response body using the ApiResponse struct
        let response_body: ApiResponse<_, String> = ApiResponse::<_, String> {
            success: false,
            message: error_message,
            data: None::<String>,
            error: Some(error_details),
        };

        //build up the response status code and the response content
        (status_code, Json(response_body)).into_response()
    }
}

///  a trait to return the field of the structs as an array of strings
///  the implementation on user information will return the user is, firstname, username ...
/// on the user authentication struct, the implementation will return the user email and password

pub trait EnumerateFields {
    fn collect_as_strings(&self) -> std::collections::HashMap<String, String>;
}
