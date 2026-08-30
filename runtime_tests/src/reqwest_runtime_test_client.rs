#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(crate) struct ReqwestRuntimeTestClient(reqwest::blocking::Client);

impl ReqwestRuntimeTestClient {
    // The owner module retains lint-sensitive semantics from the original implementation.

    pub(crate) fn send_get(
        &self,
        url: &crate::runtime_test_url::RuntimeTestUrl,
    ) -> Result<
        crate::reqwest_runtime_test_response::ReqwestRuntimeTestResponse,
        server_runtime_http::reqwest_error::ReqwestError,
    > {
        self.0
            .get(url.as_ref())
            .send()
            .map(crate::reqwest_runtime_test_response::ReqwestRuntimeTestResponse::from)
            .map_err(server_runtime_http::reqwest_error::ReqwestError::from)
    }

    // The owner module retains lint-sensitive semantics from the original implementation.

    pub(crate) fn send_notification(
        &self,
        url: &crate::runtime_test_url::RuntimeTestUrl,
        request: &notification_service_contract::create_notification_req::CreateNotificationReq,
    ) -> Result<
        crate::reqwest_runtime_test_response::ReqwestRuntimeTestResponse,
        server_runtime_http::reqwest_error::ReqwestError,
    > {
        self.0
            .post(url.as_ref())
            .json(request)
            .send()
            .map(crate::reqwest_runtime_test_response::ReqwestRuntimeTestResponse::from)
            .map_err(server_runtime_http::reqwest_error::ReqwestError::from)
    }
}
