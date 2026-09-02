#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum RuntimeTestError {
    #[error("runtime service URL is invalid: {0}")]
    BaseUrl(#[from] crate::service_base_url_error::ServiceBaseUrlError),
    #[error("runtime HTTP client could not be built: {0}")]
    Client(#[source] server_runtime_http::reqwest_error::ReqwestError),
    #[error("runtime notification test message is invalid: {0}")]
    NotificationMessage(
        #[source]
        notification_service_contract::notification_message_try_from_string_error::NotificationMessageTryFromStringError,
    ),
    #[error("runtime test report exceeded its result capacity: {0}")]
    Report(#[source] bounded_types::bounded_value_error::BoundedValueError),
    #[error("{test} request failed: {source}")]
    Request {
        #[source]
        source: server_runtime_http::reqwest_error::ReqwestError,
        test: crate::runtime_test_kind::RuntimeTestKind,
    },
    #[error("{test} response could not be decoded: {source}")]
    Response {
        #[source]
        source: server_runtime_http::reqwest_error::ReqwestError,
        test: crate::runtime_test_kind::RuntimeTestKind,
    },
    #[error("{test} returned HTTP {actual}; expected {expected}")]
    Status {
        actual: crate::http_runtime_test_status::HttpRuntimeTestStatus,
        expected: crate::http_runtime_test_status::HttpRuntimeTestStatus,
        test: crate::runtime_test_kind::RuntimeTestKind,
    },
    #[error("{test} reported an unhealthy service")]
    Unhealthy {
        test: crate::runtime_test_kind::RuntimeTestKind,
    },
}
