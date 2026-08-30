#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpFallbackRequestPathRef<'value_lt>(&'value_lt str);
impl<'value_lt> HttpFallbackRequestPathRef<'value_lt> {
    #[must_use]
    pub(crate) const fn get(self) -> &'value_lt str {
        self.0
    }
}
