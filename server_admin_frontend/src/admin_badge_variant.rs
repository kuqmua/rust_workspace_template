#[derive(
    Debug, optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Default, PartialEq, Eq,
)]
pub enum AdminBadgeVariant {
    #[default]
    Neutral,
    Success,
}

impl AdminBadgeVariant {
    pub(super) const fn class(self) -> &'static str {
        match self {
            Self::Neutral => constants_str::test_fixtures::VALUE_5386B853,
            Self::Success => constants_str::test_fixtures::VALUE_01AFB233,
        }
    }
}
