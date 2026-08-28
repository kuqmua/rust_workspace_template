#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::Display,
    newtype::FromInner,
)]
pub struct AdminCleanupRows(u64);
impl std::ops::Add for AdminCleanupRows {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0.saturating_add(rhs.0))
    }
}
impl AdminCleanupRows {
    pub(crate) const fn get(self) -> u64 {
        self.0
    }

    pub(crate) fn saturating_add(self, rhs: Self) -> Self {
        Self::from(self.0.saturating_add(rhs.0))
    }
}
