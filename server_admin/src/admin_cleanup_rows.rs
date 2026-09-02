#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::Display,
    proc_macro_newtype::FromInner,
    proc_macro_getters::Getters,
)]
pub struct AdminCleanupRows(u64);
impl std::ops::Add for AdminCleanupRows {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.get_inner().saturating_add(*rhs.get_inner()))
    }
}
impl AdminCleanupRows {
    pub(crate) const fn get(self) -> u64 {
        *self.get_inner()
    }

    pub(crate) fn saturating_add(self, rhs: Self) -> Self {
        Self::from(self.get_inner().saturating_add(*rhs.get_inner()))
    }
}
