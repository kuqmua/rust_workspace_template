#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "component props and wire enum variants retain their semantic presentation order"
)]
#[derive(
    Debug, optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Default, PartialEq, Eq,
)]
pub enum AdminInputKind {
    #[default]
    Text,
    Password,
    Number,
    Url,
}

impl AdminInputKind {
    pub(super) const fn value(self) -> &'static str {
        match self {
            Self::Text => constants_str::catalog::PG_CRUD_PG_TEXT,
            Self::Password => constants_str::catalog::PASSWORD,
            Self::Number => constants_str::test_fixtures::VALUE_12886F9D,
            Self::Url => constants_str::test_fixtures::VALUE_28E5EBAB,
        }
    }
}
