#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct HttpMetricsPathEntriesRwLock(
    std::sync::RwLock<
        std::collections::HashMap<
            crate::http_metrics_path_text::HttpMetricsPathText,
            crate::metrics_shared_string::MetricsSharedString,
        >,
    >,
);
