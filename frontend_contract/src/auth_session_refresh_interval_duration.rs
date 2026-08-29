#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthSessionRefreshIntervalDuration(pub(super) std::time::Duration);

impl TryFrom<std::time::Duration> for AuthSessionRefreshIntervalDuration {
    type Error = crate::auth_session_keep_alive_error::AuthSessionKeepAliveError;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(crate::auth_session_keep_alive_error::AuthSessionKeepAliveError::ZeroInterval)
        } else {
            Ok(Self(value))
        }
    }
}
