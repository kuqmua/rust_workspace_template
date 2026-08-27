#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpRequestPathRef<'value_lt>(pub(super) &'value_lt str);
