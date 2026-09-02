#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_getters::Getters,
)]
pub struct StdAdminFailureThreshold(
    server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64,
);
impl TryFrom<i64> for StdAdminFailureThreshold {
    type Error = crate::admin_auth_positive_value_error::AdminAuthPositiveValueError;

    fn try_from(i64: i64) -> Result<Self, Self::Error> {
        server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64::try_from(i64)
            .map(Self)
            .map_err(crate::admin_auth_positive_value_error::AdminAuthPositiveValueError::from)
    }
}
impl StdAdminFailureThreshold {
    pub(crate) const fn get(self) -> i64 {
        self.get_inner().get()
    }
}
