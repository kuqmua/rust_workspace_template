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
