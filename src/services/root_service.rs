use crate::errors::app_error::AppError;

#[derive(Clone)]
pub struct RootService {}

impl RootService {
    pub fn init() -> Self {
        Self {}
    }
}
pub trait RootServiceTrait {
    fn shut_down(&self) -> Result<(), AppError>;
    fn health_check(&self) -> Result<(), AppError>;
}

impl RootServiceTrait for RootService {
    fn shut_down(&self) -> Result<(), AppError> {
        log::info!("shutting down ...");
        std::process::exit(0);
    }
    fn health_check(&self) -> Result<(), AppError> {
        log::info!("application healthy ...");
        Ok(())
    }
}
