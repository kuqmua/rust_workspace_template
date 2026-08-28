#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirstCommaStripped(bool);
impl From<bool> for FirstCommaStripped {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl std::ops::Not for FirstCommaStripped {
    type Output = bool;
    fn not(self) -> Self::Output {
        !self.0
    }
}
