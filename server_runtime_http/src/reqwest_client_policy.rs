#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct ReqwestClientPolicy {
    connect_timeout: super::reqwest_connect_timeout_duration::ReqwestConnectTimeoutDuration,
    request_timeout: super::reqwest_request_timeout_duration::ReqwestRequestTimeoutDuration,
}

impl ReqwestClientPolicy {
    pub(crate) const fn connect_timeout(
        self,
    ) -> super::reqwest_connect_timeout_duration::ReqwestConnectTimeoutDuration {
        self.connect_timeout
    }

    #[must_use]
    pub const fn new(
        connect_timeout: super::reqwest_connect_timeout_duration::ReqwestConnectTimeoutDuration,
        request_timeout: super::reqwest_request_timeout_duration::ReqwestRequestTimeoutDuration,
    ) -> Self {
        Self {
            connect_timeout,
            request_timeout,
        }
    }

    pub(crate) const fn request_timeout(
        self,
    ) -> super::reqwest_request_timeout_duration::ReqwestRequestTimeoutDuration {
        self.request_timeout
    }
}
