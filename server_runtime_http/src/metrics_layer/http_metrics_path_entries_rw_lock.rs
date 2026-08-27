#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct HttpMetricsPathEntriesRwLock(
    pub(super)  std::sync::RwLock<
        std::collections::HashMap<super::HttpMetricsPathText, super::MetricsSharedString>,
    >,
);
