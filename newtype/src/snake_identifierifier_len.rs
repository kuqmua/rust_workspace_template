#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct SnakeIdentifierifierLen(usize);
impl From<usize> for SnakeIdentifierifierLen {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl SnakeIdentifierifierLen {
    pub(super) const fn get(&self) -> usize {
        self.0
    }
}
