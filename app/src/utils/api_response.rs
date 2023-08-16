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
/// However the two types has been merged into one type  ApiResponse<Data, Error>
/// in addition to being used independently
///Api Response definition
///
/// #Example
/// use crate::ApiResponse
/// let success :ApiResponse<Data, _>
/// let error : ApiResponse<_, Error>
/// let neither_data_nor_error : ApiResponse<_,_>
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<Data> {
    pub success: bool,
    pub message: String,
    pub data: Option<Data>,
    // pub error: Option<Error>,
}
///Api success response
/// the api success response returns succes
/// accepts message and data from handle/controller
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse<D: Serialize> {
    pub success: bool,
    pub message: String,
    pub data: Option<D>,
}

impl<D: Serialize> SuccessResponse<D> {
    pub async fn new(message: &str, data: D) -> SuccessResponse<D> {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }
}

/// the error content should be returned as an error of string
#[allow(dead_code)]
pub enum ErrorResponse {
    /// wrong authorization payload e.g incorrect username and password
    WrongCredentials { message: String },
    /// missing or wrong fields in API request
    BadRequest { message: String },
    ///internal server error
    ServerError { message: String },
    ///conflict error
    ConflictError { message: String },
    /// invalid Authorization token
    InvalidToken { message: String },
    ///missing or undefined resource e.g user information
    NotFound { message: String },
    /// authorization error
    Unauthorized { message: String },
}

///implement into response trait for API error
impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            ErrorResponse::WrongCredentials { message } => {
                //missing Authorization credentials
                (StatusCode::UNAUTHORIZED, message)
            }
            ErrorResponse::Unauthorized { message } => {
                //missing Authorization credentials
                (StatusCode::UNAUTHORIZED, message)
            }
            ErrorResponse::BadRequest { message } => (StatusCode::BAD_REQUEST, message),
            ErrorResponse::ServerError { message } => (StatusCode::INTERNAL_SERVER_ERROR, message),
            ErrorResponse::InvalidToken { message } => (StatusCode::UNAUTHORIZED, message),
            ErrorResponse::ConflictError { message } => (StatusCode::CONFLICT, message),
            //not found error
            ErrorResponse::NotFound { message } => (StatusCode::NOT_FOUND, message),
        };
        //build the response body using the ApiResponse struct
        let response_body: ApiResponse<String> = ApiResponse::<String> {
            success: false,
            message: error_message,
            data: None,
        };

        //build up the response status code and the response content
        (status_code, Json(response_body)).into_response()
    }
}

///  a trait to return the field of the structs as an array of strings
///  the implementation on user information will return the user is, firstname, username ...
/// on the user authentication struct, the implementation will return the user email and password
///
/// # example
/// ```rust
///
///   //destructure the HTTP request body
///   let UserInformation { fullname, password, username, email, } = &payload;
///
///
///  // check through the fields to see that no field was badly formatted
///  let entries = &payload.collect_as_strings();
///  let mut bad_request_errors: Vec<String> = Vec::new();
///
///  for (key, value) in entries {
///   if value.is_empty() {
///   let error = format!("{key} is empty");
///   bad_request_errors.push(error);
///  }
/// }
///
/// // do something with the response
/// ```
pub trait EnumerateFields {
    fn collect_as_strings(&self) -> std::collections::HashMap<String, String>;
}

/// a struct to extract query parameters
/// the page field represent s the current page
/// the no_of_rows will be converted to camelCase nd will be deserialized as noOfRows
/// the no of rows represents the number of items to return for the query, defaults to 10
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    /// the page number. It maps to `current page number` on the user interface
    pub page: i32,
    /// the number of items to
    pub no_of_rows: i32,
}

/// the default values of pagination
/// the default page number is set to 1 and the default number of rows is set to 10
///
/// #example
/// an example implementation in a todo handler
///
/// ```rust
/// pub async fn get_all_todo(
/// pagination: Option<Query<Pagination>>, ...other extensions )
/// -> Result<(StatusCode, Json<ApiSuccessResponse<Value>>), ErrorResponse> {
///
/// // try and get the quey params or deflect to default
///  let Query(pagination) = pagination.unwrap_or_default();
///
/// //destructure the values
/// let Pagination {
///  page: current_page,
///no_of_rows,
///  } = &pagination;
///
/// // do something else with the data
/// println!(" the current page is{current_page}, and number of rows is {no_or_rows}")
///
impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            no_of_rows: 10,
        }
    }
}
