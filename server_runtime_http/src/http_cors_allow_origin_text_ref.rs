#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpCorsAllowOriginTextRef<'text_lt>(&'text_lt str);
impl<'text_lt> HttpCorsAllowOriginTextRef<'text_lt> {
    #[must_use]
    pub(crate) const fn get(self) -> &'text_lt str {
        self.0
    }
}
