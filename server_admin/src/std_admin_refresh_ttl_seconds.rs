#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub struct StdAdminRefreshTtlSeconds(std::num::NonZeroU64);
impl TryFrom<u64> for StdAdminRefreshTtlSeconds {
    type Error = crate::admin_auth_positive_value_error::AdminAuthPositiveValueError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        crate::std_admin_auth_ttl_seconds::StdAdminAuthTtlSeconds::try_from(value)
            .map(std::num::NonZeroU64::from)
            .map(Self::from)
    }
}
impl StdAdminRefreshTtlSeconds {
    pub(crate) const fn get(self) -> u64 {
        self.get_inner().get()
    }
}
