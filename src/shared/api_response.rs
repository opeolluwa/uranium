use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
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
/// accepts message and data from handle/controller
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiSuccessResponse<Data> {
    pub success: bool,
    pub message: String,
    pub data: Option<Data>,
}

///ApiErrorResponse
pub enum ApiErrorResponse {
    WrongCredentials,
    BadRequest,
    TokenCreation,
    InvalidToken,
}

///implement into response trait for api error
impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            ApiErrorResponse::WrongCredentials => {
                //missing Authorization credentials
                (
                    StatusCode::UNAUTHORIZED,
                    String::from("Wrong or missing authorization credentials"),
                )
            }
            ApiErrorResponse::BadRequest => (
                StatusCode::BAD_REQUEST,
                String::from("Badly formatted or missing credentials"),
            ),
            ApiErrorResponse::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal Server Response"),
            ),
            ApiErrorResponse::InvalidToken => (
                StatusCode::BAD_REQUEST,
                String::from("Invalid token or missing authorization token"),
            ),
        };
        //build the response body using the ApiResponse struct
        let response_body: ApiResponse<_, String> = ApiResponse::<_, String> {
            success: false,
            message: error_message,
            data: None::<String>,
            error: None,
        };

        //build up the response status code and the response content
        (status_code, Json(response_body)).into_response()
    }
}
