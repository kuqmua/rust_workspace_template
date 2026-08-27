#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct FilterSpecValid(bool);

impl FilterSpecValid {
    pub(crate) const fn get(self) -> bool {
        self.0
    }
}
