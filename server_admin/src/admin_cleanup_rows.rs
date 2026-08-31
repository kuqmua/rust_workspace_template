#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::Display,
    newtype::FromInner,
    generate_accessor::Getters,
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
