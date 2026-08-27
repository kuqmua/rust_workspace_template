use super::AdminCleanupCfgError;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupRetentionSeconds(server_admin_contract::domain_types::PositiveNonZeroI64);
impl TryFrom<i64> for AdminCleanupRetentionSeconds {
    type Error = AdminCleanupCfgError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        server_admin_contract::domain_types::PositiveNonZeroI64::try_from(value)
            .map(Self)
            .map_err(AdminCleanupCfgError::from)
    }
}
impl AdminCleanupRetentionSeconds {
    pub(crate) const fn get(self) -> i64 {
        self.0.get()
    }
}
