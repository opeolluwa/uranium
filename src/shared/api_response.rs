use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<D> {
    pub success: bool,
    pub message: String,
    pub data: Option<D>,
    pub error: Option<D>,
    #[serde(rename = "statusCode")]
    pub status_code: Option<u32>,
}

//response builder
// impl<D: Copy> ApiResponse<D> {
//     //success response
//     pub fn success(&self) -> Self {
//         ApiResponse::<D> {
//             success: self.success,
//             message: self.message.clone(),
//             data: self.data,
//             error: None,
//             status_code: None,
//         }
//     }

//     //error response builder
//     // pub fn error(&self) -> Self {}
// }
