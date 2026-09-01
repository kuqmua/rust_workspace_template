#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype_foundation::FromInner)]
pub struct FirstCommaStripped(bool);
impl std::ops::Not for FirstCommaStripped {
    type Output = bool;
    fn not(self) -> Self::Output {
        !self.0
    }
}
