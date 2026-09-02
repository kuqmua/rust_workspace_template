#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::GetInner,
)]
pub struct AuthSessionRefreshIntervalDuration(std::time::Duration);

impl TryFrom<std::time::Duration> for AuthSessionRefreshIntervalDuration {
    type Error = crate::auth_session_keep_alive_error::AuthSessionKeepAliveError;
    fn try_from(duration: std::time::Duration) -> Result<Self, Self::Error> {
        if duration.is_zero() {
            return Err(
                crate::auth_session_keep_alive_error::AuthSessionKeepAliveError::ZeroInterval,
            );
        }
        Ok(Self(duration))
    }
}
