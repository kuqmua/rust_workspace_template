#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct PanicLine(u32);

impl PanicLine {
    pub(crate) const fn get(self) -> u32 {
        self.0
    }
}
