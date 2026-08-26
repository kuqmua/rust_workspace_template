#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the HTTP client owner reads this private policy across owner modules"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct ReqwestClientPolicy {
    pub(super) connect_timeout:
        super::reqwest_connect_timeout_duration::ReqwestConnectTimeoutDuration,
    pub(super) request_timeout:
        super::reqwest_request_timeout_duration::ReqwestRequestTimeoutDuration,
}

impl ReqwestClientPolicy {
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
}
