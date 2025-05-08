use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    message: String,
    data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn from(message: &str, data: Option<T>) -> Self {
        Self {
            message: message.to_string(),
            data,
        }
    }
}



pub type HandlerResponse<T> = Json<ApiResponse<T>>;