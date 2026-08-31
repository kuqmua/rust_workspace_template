#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(super) struct MetricsSharedString(metrics::SharedString);

impl MetricsSharedString {
    #[cfg(test)]
    pub(super) fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    pub(super) fn into_inner(self) -> metrics::SharedString {
        self.0
    }
}
