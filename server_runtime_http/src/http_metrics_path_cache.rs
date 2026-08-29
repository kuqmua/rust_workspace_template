#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(super) struct HttpMetricsPathCache {
    pub(super) entries: crate::http_metrics_path_entries_rw_lock::HttpMetricsPathEntriesRwLock,
    pub(super) maximum: crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum,
    pub(super) unmatched: crate::metrics_shared_string::MetricsSharedString,
}

impl HttpMetricsPathCache {
    // The owner module retains lint-sensitive semantics from the original implementation.

    #[cfg(test)]
    pub(super) fn new(
        maximum: crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum,
    ) -> Self {
        Self {
            entries: crate::http_metrics_path_entries_rw_lock::HttpMetricsPathEntriesRwLock::from(
                std::sync::RwLock::new(std::collections::HashMap::with_capacity(
                    maximum.0.get().min(constants_usize::VALUE_4_096),
                )),
            ),
            maximum,
            unmatched: crate::metrics_shared_string::MetricsSharedString::from(
                metrics::SharedString::const_str(
                    constants_str::catalog::HTTP_METRICS_UNMATCHED_PATH,
                ),
            ),
        }
    }

    pub(super) fn label(
        &self,
        path: crate::http_metrics_path_text_ref::HttpMetricsPathTextRef<'_>,
    ) -> crate::metrics_shared_string::MetricsSharedString {
        {
            let read_entries = self
                .entries
                .0
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(label) = read_entries.get(path.0) {
                return label.clone();
            }
            if read_entries.len() >= self.maximum.0.get() {
                return self.unmatched.clone();
            }
        }
        let mut write_entries = self
            .entries
            .0
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(label) = write_entries.get(path.0) {
            return label.clone();
        }
        if write_entries.len() >= self.maximum.0.get() {
            return self.unmatched.clone();
        }
        let Ok(path_text) =
            crate::http_metrics_path_text::HttpMetricsPathText::try_from(path.0.to_owned())
        else {
            return self.unmatched.clone();
        };
        let label = crate::metrics_shared_string::MetricsSharedString::from(
            metrics::SharedString::from(path_text.0.clone()),
        );
        let _previous = write_entries.insert(path_text, label.clone());
        label
    }
}
