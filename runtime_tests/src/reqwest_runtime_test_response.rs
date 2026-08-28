#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct ReqwestRuntimeTestResponse(reqwest::blocking::Response);

impl ReqwestRuntimeTestResponse {
    // The owner module retains lint-sensitive semantics from the original implementation.

    pub(crate) fn into_health_report(
        self,
    ) -> Result<
        common_routes::domain_types::HealthReport,
        server_runtime_http::domain_types::ReqwestError,
    > {
        self.0
            .json::<common_routes::domain_types::HealthReport>()
            .map_err(server_runtime_http::domain_types::ReqwestError::from)
    }
    // The owner module retains lint-sensitive semantics from the original implementation.

    pub(crate) fn into_notification_res(
        self,
    ) -> Result<
        notification_service_contract::domain_types::CreateNotificationRes,
        server_runtime_http::domain_types::ReqwestError,
    > {
        self.0
            .json::<notification_service_contract::domain_types::CreateNotificationRes>()
            .map_err(server_runtime_http::domain_types::ReqwestError::from)
    }
    #[must_use]
    pub(crate) fn status(&self) -> crate::domain_types::HttpRuntimeTestStatus {
        crate::domain_types::HttpRuntimeTestStatus::from(self.0.status().as_u16())
    }
}
