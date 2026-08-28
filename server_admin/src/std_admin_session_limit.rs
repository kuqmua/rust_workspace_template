#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::AdminAuthPositiveValueError;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct StdAdminSessionLimit(pub(crate) std::num::NonZeroUsize);
impl TryFrom<usize> for StdAdminSessionLimit {
    type Error = AdminAuthPositiveValueError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self::from)
            .ok_or(AdminAuthPositiveValueError)
    }
}
impl From<std::num::NonZeroUsize> for StdAdminSessionLimit {
    fn from(value: std::num::NonZeroUsize) -> Self {
        Self(value)
    }
}
impl StdAdminSessionLimit {
    pub(crate) const fn get(self) -> usize {
        self.0.get()
    }
}
