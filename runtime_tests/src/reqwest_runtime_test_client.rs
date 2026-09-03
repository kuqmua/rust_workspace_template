#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub(crate) struct ReqwestRuntimeTestClient(reqwest::blocking::Client);

impl ReqwestRuntimeTestClient {
    pub(crate) fn send_get(
        &self,
        runtime_test_url: &crate::runtime_test_url::RuntimeTestUrl,
    ) -> Result<
        crate::reqwest_runtime_test_response::ReqwestRuntimeTestResponse,
        server_runtime_http::reqwest_error::ReqwestError,
    > {
        self.0
            .get(runtime_test_url.as_ref())
            .send()
            .map(crate::reqwest_runtime_test_response::ReqwestRuntimeTestResponse::from)
            .map_err(server_runtime_http::reqwest_error::ReqwestError::from)
    }

    pub(crate) fn send_notification(
        &self,
        runtime_test_url: &crate::runtime_test_url::RuntimeTestUrl,
        create_notification_request: &notification_service_contract::create_notification_request::CreateNotificationRequest,
    ) -> Result<
        crate::reqwest_runtime_test_response::ReqwestRuntimeTestResponse,
        server_runtime_http::reqwest_error::ReqwestError,
    > {
        self.0
            .post(runtime_test_url.as_ref())
            .json(create_notification_request)
            .send()
            .map(crate::reqwest_runtime_test_response::ReqwestRuntimeTestResponse::from)
            .map_err(server_runtime_http::reqwest_error::ReqwestError::from)
    }
}
