#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct TrustedProxyRangesTextRef<'text_lt>(pub(super) &'text_lt str);
