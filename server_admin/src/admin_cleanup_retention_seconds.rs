#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
)]
pub struct AdminCleanupRetentionSeconds(
    server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64,
);
impl TryFrom<i64> for AdminCleanupRetentionSeconds {
    type Error = crate::admin_cleanup_configuration_error::AdminCleanupConfigurationError;
    fn try_from(i64: i64) -> Result<Self, Self::Error> {
        server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64::try_from(i64)
            .map(Self)
            .map_err(crate::admin_cleanup_configuration_error::AdminCleanupConfigurationError::from)
    }
}
impl AdminCleanupRetentionSeconds {
    pub(crate) const fn get(self) -> i64 {
        self.get_inner().get()
    }
}
