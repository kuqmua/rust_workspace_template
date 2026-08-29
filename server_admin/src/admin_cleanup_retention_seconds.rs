#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupRetentionSeconds(
    server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64,
);
impl TryFrom<i64> for AdminCleanupRetentionSeconds {
    type Error = crate::admin_cleanup_cfg_error::AdminCleanupCfgError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64::try_from(value)
            .map(Self)
            .map_err(crate::admin_cleanup_cfg_error::AdminCleanupCfgError::from)
    }
}
impl AdminCleanupRetentionSeconds {
    pub(crate) const fn get(self) -> i64 {
        self.0.get()
    }
}
