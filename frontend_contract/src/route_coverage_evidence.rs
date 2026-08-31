#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    generate_accessor::Getters,
    generate_constructor::New,
)]
pub struct RouteCoverageEvidence {
    obligations: &'static [crate::route_coverage_obligation::RouteCoverageObligation],
}
