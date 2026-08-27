#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::RouteCoverageObligation;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteCoverageEvidence {
    pub(super) obligations: &'static [RouteCoverageObligation],
}

impl RouteCoverageEvidence {
    #[must_use]
    pub const fn new(obligations: &'static [RouteCoverageObligation]) -> Self {
        Self { obligations }
    }
}
