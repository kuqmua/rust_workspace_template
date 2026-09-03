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
pub struct StdAdminAccessTtlSeconds(std::num::NonZeroU64);
impl TryFrom<u64> for StdAdminAccessTtlSeconds {
    type Error = crate::admin_auth_positive_value_error::AdminAuthPositiveValueError;

    fn try_from(u64: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(u64)
            .map(Self::from)
            .ok_or(crate::admin_auth_positive_value_error::AdminAuthPositiveValueError::Zero)
    }
}
impl StdAdminAccessTtlSeconds {
    pub(crate) const fn get(self) -> u64 {
        self.get_inner().get()
    }
}
