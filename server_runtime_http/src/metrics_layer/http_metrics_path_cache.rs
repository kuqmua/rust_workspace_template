#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(super) struct HttpMetricsPathCache {
    entries: super::HttpMetricsPathEntriesRwLock,
    maximum: super::HttpMetricsPathCacheMaximum,
    unmatched: super::MetricsSharedString,
}

impl HttpMetricsPathCache {
    #[allow(clippy::single_call_fn)]
    pub(super) fn new(maximum: super::HttpMetricsPathCacheMaximum) -> Self {
        Self {
            entries: super::HttpMetricsPathEntriesRwLock::from(std::sync::RwLock::new(
                std::collections::HashMap::with_capacity(
                    maximum
                        .0
                        .0
                        .get()
                        .min(super::DEFAULT_HTTP_METRICS_PATH_CACHE_MAXIMUM),
                ),
            )),
            maximum,
            unmatched: super::MetricsSharedString::from(metrics::SharedString::const_str(
                constants_str::HTTP_METRICS_UNMATCHED_PATH,
            )),
        }
    }

    pub(super) fn label(
        &self,
        path: super::HttpMetricsPathTextRef<'_>,
    ) -> super::MetricsSharedString {
        {
            let read_entries = self
                .entries
                .0
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(label) = read_entries.get(path.0) {
                return label.clone();
            }
            if read_entries.len() >= self.maximum.0.0.get() {
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
        if write_entries.len() >= self.maximum.0.0.get() {
            return self.unmatched.clone();
        }
        let Ok(path_text) = super::HttpMetricsPathText::try_from(path.0.to_owned()) else {
            return self.unmatched.clone();
        };
        let label =
            super::MetricsSharedString::from(metrics::SharedString::from(path_text.0.clone()));
        let _previous = write_entries.insert(path_text, label.clone());
        label
    }
}
