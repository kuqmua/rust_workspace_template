#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(super) struct MetricsSharedString(pub(super) metrics::SharedString);
