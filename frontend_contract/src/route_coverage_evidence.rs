#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub struct RouteCoverageEvidence {
    obligations: &'static [crate::route_coverage_obligation::RouteCoverageObligation],
}
