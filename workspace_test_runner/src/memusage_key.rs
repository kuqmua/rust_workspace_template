#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct MemusageKey(&'static str);
impl MemusageKey {
    pub(crate) const fn get(self) -> &'static str {
        self.0
    }
}
