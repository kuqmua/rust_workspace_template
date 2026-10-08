#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    Default,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
pub(super) struct AnalyzerCount(usize);
impl AnalyzerCount {
    pub(super) fn saturating_dec(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }
    pub(super) fn saturating_inc(&mut self) {
        self.0 = self.0.saturating_add(1);
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_analyzer_count_defaults_and_exact_saturation_boundaries() {
        assert_eq!(super::AnalyzerCount::default().get(), 0usize);
        [
            (0usize, 0usize, 1usize),
            (1usize, 0usize, 2usize),
            (2usize, 1usize, 3usize),
            (usize::MAX - 1usize, usize::MAX - 2usize, usize::MAX),
            (usize::MAX, usize::MAX - 1usize, usize::MAX),
        ]
        .into_iter()
        .fold((), |(), (value, decremented, incremented)| {
            let mut analyzer_count = super::AnalyzerCount::from(value);
            assert_eq!(analyzer_count.get(), value);
            analyzer_count.saturating_dec();
            assert_eq!(analyzer_count.get(), decremented);
            analyzer_count = super::AnalyzerCount::from(value);
            analyzer_count.saturating_inc();
            assert_eq!(analyzer_count.get(), incremented);
        });
    }
}
