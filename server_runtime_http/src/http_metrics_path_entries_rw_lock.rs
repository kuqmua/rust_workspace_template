#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct HttpMetricsPathEntriesRwLock(
    std::sync::RwLock<
        std::collections::HashMap<
            crate::http_metrics_path_text::HttpMetricsPathText,
            crate::metrics_shared_string::MetricsSharedString,
        >,
    >,
);
