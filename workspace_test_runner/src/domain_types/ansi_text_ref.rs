#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct AnsiTextRef<'lt>(&'lt str);
impl<'lt> AnsiTextRef<'lt> {
    pub(crate) const fn get(self) -> &'lt str {
        self.0
    }
}
