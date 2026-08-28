#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(crate) struct ReqwestRuntimeTestClient(reqwest::blocking::Client);

impl ReqwestRuntimeTestClient {
    // The owner module retains lint-sensitive semantics from the original implementation.

    pub(crate) fn send_get(
        &self,
        url: &crate::domain_types::RuntimeTestUrl,
    ) -> Result<
        crate::domain_types::ReqwestRuntimeTestResponse,
        server_runtime_http::domain_types::ReqwestError,
    > {
        self.0
            .get(url.0.as_str())
            .send()
            .map(crate::domain_types::ReqwestRuntimeTestResponse::from)
            .map_err(server_runtime_http::domain_types::ReqwestError::from)
    }

    // The owner module retains lint-sensitive semantics from the original implementation.

    pub(crate) fn send_notification(
        &self,
        url: &crate::domain_types::RuntimeTestUrl,
        request: &notification_service_contract::domain_types::CreateNotificationReq,
    ) -> Result<
        crate::domain_types::ReqwestRuntimeTestResponse,
        server_runtime_http::domain_types::ReqwestError,
    > {
        self.0
            .post(url.0.as_str())
            .json(request)
            .send()
            .map(crate::domain_types::ReqwestRuntimeTestResponse::from)
            .map_err(server_runtime_http::domain_types::ReqwestError::from)
    }
}
