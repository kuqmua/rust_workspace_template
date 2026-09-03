#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, proc_macro_newtype::FromInner,
)]
pub(crate) struct ReqwestRuntimeTestResponse(reqwest::blocking::Response);

impl ReqwestRuntimeTestResponse {
    pub(crate) fn into_health_report(
        self,
    ) -> Result<
        common_routes::health_report::HealthReport,
        server_runtime_http::reqwest_error::ReqwestError,
    > {
        self.0
            .json::<common_routes::health_report::HealthReport>()
            .map_err(server_runtime_http::reqwest_error::ReqwestError::from)
    }

    pub(crate) fn into_notification_res(
        self,
    ) -> Result<
        notification_service_contract::create_notification_response::CreateNotificationResponse,
        server_runtime_http::reqwest_error::ReqwestError,
    > {
        self.0
            .json::<notification_service_contract::create_notification_response::CreateNotificationResponse>()
            .map_err(server_runtime_http::reqwest_error::ReqwestError::from)
    }
    #[must_use]
    pub(crate) fn status(&self) -> crate::http_runtime_test_status::HttpRuntimeTestStatus {
        crate::http_runtime_test_status::HttpRuntimeTestStatus::from(self.0.status().as_u16())
    }
}
