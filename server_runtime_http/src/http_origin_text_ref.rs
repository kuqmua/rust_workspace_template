#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct HttpOriginTextRef<'text>(&'text str);

impl<'text> HttpOriginTextRef<'text> {
    pub(crate) const fn get(self) -> &'text str {
        self.0
    }
}
