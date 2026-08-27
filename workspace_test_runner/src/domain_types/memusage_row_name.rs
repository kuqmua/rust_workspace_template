#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct MemusageRowName(&'static str);
impl MemusageRowName {
    pub(crate) const fn get(self) -> &'static str {
        self.0
    }
}
