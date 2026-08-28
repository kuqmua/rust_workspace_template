#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct NewtypeBool(bool);
impl From<bool> for NewtypeBool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl NewtypeBool {
    pub(crate) const fn get(&self) -> bool {
        self.0
    }
}
