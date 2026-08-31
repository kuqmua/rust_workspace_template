#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(super) struct HttpMetricsPathCache {
    entries: crate::http_metrics_path_entries_rw_lock::HttpMetricsPathEntriesRwLock,
    maximum: crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum,
    unmatched: crate::metrics_shared_string::MetricsSharedString,
}

impl HttpMetricsPathCache {
    // The owner module retains lint-sensitive semantics from the original implementation.
    pub(super) fn label(
        &self,
        path: crate::http_metrics_path_text_ref::HttpMetricsPathTextRef<'_>,
    ) -> crate::metrics_shared_string::MetricsSharedString {
        {
            let read_entries = self
                .entries
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(label) = read_entries.get(*path) {
                return label.clone();
            }
            if read_entries.len() >= self.maximum.get() {
                return self.unmatched.clone();
            }
        }
        let mut write_entries = self
            .entries
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(label) = write_entries.get(*path) {
            return label.clone();
        }
        if write_entries.len() >= self.maximum.get() {
            return self.unmatched.clone();
        }
        let Ok(path_text) =
            crate::http_metrics_path_text::HttpMetricsPathText::try_from((*path).to_owned())
        else {
            return self.unmatched.clone();
        };
        let label = crate::metrics_shared_string::MetricsSharedString::from(
            metrics::SharedString::from((*path_text).clone()),
        );
        let _previous = write_entries.insert(path_text, label.clone());
        label
    }
}

impl From<crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum>
    for HttpMetricsPathCache
{
    fn from(maximum: crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum) -> Self {
        Self {
            entries: crate::http_metrics_path_entries_rw_lock::HttpMetricsPathEntriesRwLock::from(
                std::sync::RwLock::new(std::collections::HashMap::with_capacity(
                    maximum.get().min(constants_usize::VALUE_4_096),
                )),
            ),
            maximum,
            unmatched: crate::metrics_shared_string::MetricsSharedString::from(
                metrics::SharedString::const_str(constants_str::HTTP_METRICS_UNMATCHED_PATH),
            ),
        }
    }
}
