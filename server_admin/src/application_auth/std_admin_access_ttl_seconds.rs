#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{AdminAuthPositiveValueError, StdAdminAccessTtlSecondsNonZeroU64};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct StdAdminAccessTtlSeconds(pub(super) StdAdminAccessTtlSecondsNonZeroU64);
impl TryFrom<u64> for StdAdminAccessTtlSeconds {
    type Error = AdminAuthPositiveValueError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(value)
            .map(Self::from)
            .ok_or(AdminAuthPositiveValueError)
    }
}
impl From<std::num::NonZeroU64> for StdAdminAccessTtlSeconds {
    fn from(value: std::num::NonZeroU64) -> Self {
        Self(StdAdminAccessTtlSecondsNonZeroU64::from(value))
    }
}
impl StdAdminAccessTtlSeconds {
    pub(super) const fn get(self) -> u64 {
        self.0.0.get()
    }
}
