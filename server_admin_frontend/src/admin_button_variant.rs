#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "component props and wire enum variants retain their semantic presentation order"
)]
#[derive(
    Debug, optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Default, PartialEq, Eq,
)]
pub enum AdminButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
}

impl AdminButtonVariant {
    pub(super) const fn class(self) -> &'static str {
        match self {
            Self::Primary => constants_str::test_fixtures::VALUE_82FEF3B0,
            Self::Secondary => constants_str::test_fixtures::VALUE_D720672A,
            Self::Danger => constants_str::test_fixtures::VALUE_7BE8BA9D,
        }
    }
}
