#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct StdAdminAccessTtlSeconds(pub(crate) std::num::NonZeroU64);
impl TryFrom<u64> for StdAdminAccessTtlSeconds {
    type Error = crate::admin_auth_positive_value_error::AdminAuthPositiveValueError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(value)
            .map(Self::from)
            .ok_or(crate::admin_auth_positive_value_error::AdminAuthPositiveValueError)
    }
}
impl From<std::num::NonZeroU64> for StdAdminAccessTtlSeconds {
    fn from(value: std::num::NonZeroU64) -> Self {
        Self(value)
    }
}
impl StdAdminAccessTtlSeconds {
    pub(crate) const fn get(self) -> u64 {
        self.0.get()
    }
}
