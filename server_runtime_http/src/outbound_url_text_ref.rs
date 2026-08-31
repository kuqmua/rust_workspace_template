#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct OutboundUrlTextRef<'value_lt>(&'value_lt str);

impl<'value_lt> OutboundUrlTextRef<'value_lt> {
    pub(crate) const fn get(self) -> &'value_lt str {
        self.0
    }
}
