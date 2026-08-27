#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "component props and wire enum variants retain their semantic presentation order"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum AdminButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
}

impl AdminButtonVariant {
    pub(super) const fn class(self) -> &'static str {
        match self {
            Self::Primary => constants_str::VALUE_82FEF3B0,
            Self::Secondary => constants_str::VALUE_D720672A,
            Self::Danger => constants_str::VALUE_7BE8BA9D,
        }
    }
}
