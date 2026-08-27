#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct StdUniqueOptionSetContains(bool);
impl From<bool> for StdUniqueOptionSetContains {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl StdUniqueOptionSetContains {
    #[must_use]
    pub const fn get(self) -> bool {
        self.0
    }
}
