#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartIndex(usize);
impl From<usize> for PartIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl PartIndex {
    pub(super) const fn get(self) -> usize {
        self.0
    }
}
