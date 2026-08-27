#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct HttpOriginTextRef<'text>(pub(super) &'text str);
