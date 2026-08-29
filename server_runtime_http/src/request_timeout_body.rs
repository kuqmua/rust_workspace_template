#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Serialize)]
pub(super) struct RequestTimeoutBody {
    pub(super) error: crate::std_request_timeout_message::StdRequestTimeoutMessage,
}
