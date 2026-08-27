#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthSessionRefreshIntervalDuration(pub(super) std::time::Duration);

impl TryFrom<std::time::Duration> for AuthSessionRefreshIntervalDuration {
    type Error = super::AuthSessionKeepAliveError;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(super::AuthSessionKeepAliveError::ZeroInterval)
        } else {
            Ok(Self(value))
        }
    }
}
