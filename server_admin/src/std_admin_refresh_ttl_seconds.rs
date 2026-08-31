#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub struct StdAdminRefreshTtlSeconds(std::num::NonZeroU64);
impl TryFrom<u64> for StdAdminRefreshTtlSeconds {
    type Error = crate::admin_auth_positive_value_error::AdminAuthPositiveValueError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(value)
            .map(Self::from)
            .ok_or(crate::admin_auth_positive_value_error::AdminAuthPositiveValueError::Zero)
    }
}
impl StdAdminRefreshTtlSeconds {
    pub(crate) const fn get(self) -> u64 {
        self.get_inner().get()
    }
}
