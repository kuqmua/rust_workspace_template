#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct StdUniqueOptionSetIsEmpty(bool);
impl From<bool> for StdUniqueOptionSetIsEmpty {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl StdUniqueOptionSetIsEmpty {
    #[must_use]
    pub const fn get(self) -> bool {
        self.0
    }
}
