#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::AdminAuthPositiveValueError;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct StdAdminFailureThreshold(
    pub(super) server_admin_contract::domain_types::PositiveNonZeroI64,
);
impl TryFrom<i64> for StdAdminFailureThreshold {
    type Error = AdminAuthPositiveValueError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        server_admin_contract::domain_types::PositiveNonZeroI64::try_from(value)
            .map(Self)
            .map_err(AdminAuthPositiveValueError::from)
    }
}
impl StdAdminFailureThreshold {
    pub(crate) const fn get(self) -> i64 {
        self.0.get()
    }
}
