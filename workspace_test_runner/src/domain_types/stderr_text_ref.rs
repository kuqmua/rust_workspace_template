#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct StderrTextRef<'lt>(&'lt str);
impl<'lt> StderrTextRef<'lt> {
    pub(crate) const fn get(self) -> &'lt str {
        self.0
    }
}
