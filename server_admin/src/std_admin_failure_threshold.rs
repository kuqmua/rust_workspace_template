#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct StdAdminFailureThreshold(
    pub(crate) server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64,
);
impl TryFrom<i64> for StdAdminFailureThreshold {
    type Error = crate::admin_auth_positive_value_error::AdminAuthPositiveValueError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64::try_from(value)
            .map(Self)
            .map_err(crate::admin_auth_positive_value_error::AdminAuthPositiveValueError::from)
    }
}
impl StdAdminFailureThreshold {
    pub(crate) const fn get(self) -> i64 {
        self.0.get()
    }
}
