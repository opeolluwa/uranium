#[derive(thiserror::Error, Debug)]
pub enum UserServiceError {
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Invalid hash")]
    InvalidHash,
    #[error("{0}")]
    OperationFailed(String),
}
