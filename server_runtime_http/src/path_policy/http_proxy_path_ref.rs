#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpProxyPathRef<'value_lt>(pub(super) &'value_lt str);
