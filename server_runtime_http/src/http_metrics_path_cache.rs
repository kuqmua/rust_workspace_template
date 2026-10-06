#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(super) struct HttpMetricsPathCache {
    entries: crate::http_metrics_path_entries_rw_lock::HttpMetricsPathEntriesRwLock,
    maximum: crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum,
    unmatched: crate::metrics_shared_string::MetricsSharedString,
}

impl HttpMetricsPathCache {
    pub(super) fn label(
        &self,
        http_metrics_path_text_ref: crate::http_metrics_path_text_ref::HttpMetricsPathTextRef<'_>,
    ) -> crate::metrics_shared_string::MetricsSharedString {
        {
            let read_entries = self
                .entries
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(label) = read_entries.get(*http_metrics_path_text_ref) {
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
        if let Some(label) = write_entries.get(*http_metrics_path_text_ref) {
            return label.clone();
        }
        if write_entries.len() >= self.maximum.get() {
            return self.unmatched.clone();
        }
        let Ok(path_text) = crate::http_metrics_path_text::HttpMetricsPathText::try_from(
            (*http_metrics_path_text_ref).to_owned(),
        ) else {
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
    fn from(value: crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum) -> Self {
        Self {
            entries: crate::http_metrics_path_entries_rw_lock::HttpMetricsPathEntriesRwLock::from(
                std::sync::RwLock::new(std::collections::HashMap::with_capacity(
                    value.get().min(constants_usize::VALUE_4_096),
                )),
            ),
            maximum: value,
            unmatched: crate::metrics_shared_string::MetricsSharedString::from(
                metrics::SharedString::const_str(constants_str::HTTP_METRICS_UNMATCHED_PATH),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_metrics_cache_recovers_poisoned_lock_without_losing_capacity_or_labels() {
        let cache = crate::http_metrics_path_cache::HttpMetricsPathCache::from(
            crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum::from(
                std::num::NonZeroUsize::MIN,
            ),
        );
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if let Ok(write_entries) = cache.entries.write() {
                assert!(write_entries.is_empty());
                std::panic::panic_any(());
            }
        }));
        assert!(outcome.is_err());
        assert!(
            [
                (constants_str::ROOT, constants_str::ROOT),
                (constants_str::ROOT, constants_str::ROOT),
                (
                    constants_str::V1,
                    constants_str::HTTP_METRICS_UNMATCHED_PATH
                ),
            ]
            .into_iter()
            .all(|(path, expected)| {
                cache
                    .label(crate::http_metrics_path_text_ref::HttpMetricsPathTextRef::from(path))
                    .as_str()
                    == expected
            })
        );
    }
}
