#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct TextRef<'text_lt>(pub(super) &'text_lt str);
impl<'text_lt> TextRef<'text_lt> {
    pub(super) const fn get(self) -> &'text_lt str {
        self.0
    }
}
