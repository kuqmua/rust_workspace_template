#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartIndex(pub(super) usize);
impl From<usize> for PartIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
