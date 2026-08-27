#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct HttpMetricsPathTextRef<'path>(pub(super) &'path str);
