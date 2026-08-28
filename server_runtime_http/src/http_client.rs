pub use crate::reqwest_client::ReqwestClient;
pub use crate::reqwest_client_build_error::ReqwestClientBuildError;
pub use crate::reqwest_client_policy::ReqwestClientPolicy;
pub use crate::reqwest_connect_timeout_duration::ReqwestConnectTimeoutDuration;
pub use crate::reqwest_request_timeout_duration::ReqwestRequestTimeoutDuration;
pub use crate::std_reqwest_timeout_error::StdReqwestTimeoutError;

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
    pub use crate::reqwest_client::*;
}
mod reqwest_client_build_error {
    pub use crate::reqwest_client_build_error::*;
}
mod reqwest_client_policy {
    pub use crate::reqwest_client_policy::*;
}
mod reqwest_connect_timeout_duration {
    pub use crate::reqwest_connect_timeout_duration::*;
}
mod reqwest_request_timeout_duration {
    pub use crate::reqwest_request_timeout_duration::*;
}
mod std_reqwest_timeout_duration_ref {
    pub use crate::std_reqwest_timeout_duration_ref::*;
}
mod std_reqwest_timeout_error {
    pub use crate::std_reqwest_timeout_error::*;
}
mod tracing_http_client_span {
    pub use crate::tracing_http_client_span::*;
}
