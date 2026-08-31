#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::DerefInner, newtype::FromInner,
)]
pub(super) struct HttpMetricsPathEntriesRwLock(
    std::sync::RwLock<
        std::collections::HashMap<
            crate::http_metrics_path_text::HttpMetricsPathText,
            crate::metrics_shared_string::MetricsSharedString,
        >,
    >,
);
