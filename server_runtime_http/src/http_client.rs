#[path = "http_client_reqwest_client.rs"]
mod reqwest_client;
#[path = "http_client_reqwest_client_build_error.rs"]
mod reqwest_client_build_error;
#[path = "http_client_reqwest_client_policy.rs"]
mod reqwest_client_policy;
#[path = "http_client_reqwest_connect_timeout_duration.rs"]
mod reqwest_connect_timeout_duration;
#[path = "http_client_reqwest_request_timeout_duration.rs"]
mod reqwest_request_timeout_duration;
#[path = "http_client_std_reqwest_timeout_error.rs"]
mod std_reqwest_timeout_error;
#[path = "http_client_tracing_http_client_span.rs"]
mod tracing_http_client_span;

pub use reqwest_client::ReqwestClient;
pub use reqwest_client_build_error::ReqwestClientBuildError;
pub use reqwest_client_policy::ReqwestClientPolicy;
pub use reqwest_connect_timeout_duration::ReqwestConnectTimeoutDuration;
pub use reqwest_request_timeout_duration::ReqwestRequestTimeoutDuration;
pub use std_reqwest_timeout_error::StdReqwestTimeoutError;

#[cfg(test)]
mod tests {
    #[test]
    fn timeout_wrappers_reject_zero() {
        assert_eq!(
            super::ReqwestConnectTimeoutDuration::try_from(std::time::Duration::ZERO).err(),
            Some(super::StdReqwestTimeoutError)
        );
        assert_eq!(
            super::ReqwestRequestTimeoutDuration::try_from(std::time::Duration::ZERO).err(),
            Some(super::StdReqwestTimeoutError)
        );
    }
}
