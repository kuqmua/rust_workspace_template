#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct MemusageProgNameRef<'lt>(&'lt str);
impl<'lt> MemusageProgNameRef<'lt> {
    pub(crate) const fn get(self) -> &'lt str {
        self.0
    }
}
