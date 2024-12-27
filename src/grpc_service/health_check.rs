use tonic::{async_trait, Response};

use crate::proto::health_check::{
    health_check_server::HealthCheck, HealthCheckRequest, HealthCheckResponse, Status,
};

#[derive(Default, Clone)]
pub struct HealthCheckImplementation {}

#[async_trait]

impl HealthCheck for HealthCheckImplementation {
    async fn check_service_health(
        &self,
        _: tonic::Request<HealthCheckRequest>,
    ) -> std::result::Result<tonic::Response<HealthCheckResponse>, tonic::Status> {
        let res = HealthCheckResponse {
            status: Status::Ok.into(),
            message: "Service up and running".to_string(),
        };

        Ok(Response::new(res))
    }
}
