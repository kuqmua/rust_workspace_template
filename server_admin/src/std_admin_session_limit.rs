#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype::FromInner,
    proc_macro_getters::Getters,
)]
pub struct StdAdminSessionLimit(std::num::NonZeroUsize);
impl TryFrom<usize> for StdAdminSessionLimit {
    type Error = crate::admin_auth_positive_value_error::AdminAuthPositiveValueError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self::from)
            .ok_or(crate::admin_auth_positive_value_error::AdminAuthPositiveValueError::Zero)
    }
}
impl StdAdminSessionLimit {
    pub(crate) const fn get(self) -> usize {
        self.get_inner().get()
    }
}
