#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    Default,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
pub(super) struct AnalyzerBool(bool);
impl AnalyzerBool {
    pub(super) fn set_true(&mut self) {
        self.0 = true;
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_analyzer_bool_defaults_and_monotonic_set_true() {
        assert!(!super::AnalyzerBool::default().get());
        [false, true].into_iter().fold((), |(), value| {
            let mut analyzer_bool = super::AnalyzerBool::from(value);
            assert_eq!(analyzer_bool.get(), value);
            analyzer_bool.set_true();
            assert!(analyzer_bool.get());
            analyzer_bool.set_true();
            assert!(analyzer_bool.get());
        });
    }
}
