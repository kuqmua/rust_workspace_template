#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
)]
pub struct RouteCoverageEvidence {
    pub(super) obligations: &'static [crate::route_coverage_obligation::RouteCoverageObligation],
}
