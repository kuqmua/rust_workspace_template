#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct PanicColumn(u32);

impl PanicColumn {
    pub(crate) const fn get(self) -> u32 {
        self.0
    }
}
