#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpAllowedPathPrefixRef<'value_lt>(pub(super) &'value_lt str);
