#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum RuntimeTestError {
    #[error("runtime service URL is invalid: {0}")]
    BaseUrl(#[from] crate::domain_types::ServiceBaseUrlError),
    #[error("runtime HTTP client could not be built: {0}")]
    Client(#[source] server_runtime_http::domain_types::ReqwestError),
    #[error("runtime notification test message is invalid: {0}")]
    NotificationMessage(
        #[source]
        notification_service_contract::domain_types::NotificationMessageTryFromStringError,
    ),
    #[error("runtime test report exceeded its result capacity: {0}")]
    Report(#[source] bounded_types::domain_types::BoundedValueError),
    #[error("{test} request failed: {source}")]
    Request {
        #[source]
        source: server_runtime_http::domain_types::ReqwestError,
        test: crate::domain_types::RuntimeTestKind,
    },
    #[error("{test} response could not be decoded: {source}")]
    Response {
        #[source]
        source: server_runtime_http::domain_types::ReqwestError,
        test: crate::domain_types::RuntimeTestKind,
    },
    #[error("{test} returned HTTP {actual}; expected {expected}")]
    Status {
        actual: crate::domain_types::HttpRuntimeTestStatus,
        expected: crate::domain_types::HttpRuntimeTestStatus,
        test: crate::domain_types::RuntimeTestKind,
    },
    #[error("{test} reported an unhealthy service")]
    Unhealthy {
        test: crate::domain_types::RuntimeTestKind,
    },
}
