pub use super::reqwest_client::ReqwestClient;
pub use super::reqwest_client_build_error::ReqwestClientBuildError;
pub use super::reqwest_client_policy::ReqwestClientPolicy;
pub use super::reqwest_connect_timeout_duration::ReqwestConnectTimeoutDuration;
pub use super::reqwest_request_timeout_duration::ReqwestRequestTimeoutDuration;
pub use super::std_reqwest_timeout_error::StdReqwestTimeoutError;
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

// Root-owned module compatibility wrappers.
mod reqwest_client {
    pub use super::super::reqwest_client::*;
}
mod reqwest_client_build_error {
    pub use super::super::reqwest_client_build_error::*;
}
mod reqwest_client_policy {
    pub use super::super::reqwest_client_policy::*;
}
mod reqwest_connect_timeout_duration {
    pub use super::super::reqwest_connect_timeout_duration::*;
}
mod reqwest_request_timeout_duration {
    pub use super::super::reqwest_request_timeout_duration::*;
}
mod std_reqwest_timeout_duration_ref {
    pub use super::super::std_reqwest_timeout_duration_ref::*;
}
mod std_reqwest_timeout_error {
    pub use super::super::std_reqwest_timeout_error::*;
}
mod tracing_http_client_span {
    pub use super::super::tracing_http_client_span::*;
}
