#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "component props and wire enum variants retain their semantic presentation order"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum AdminInputKind {
    #[default]
    Text,
    Password,
    Number,
    Url,
}

impl AdminInputKind {
    pub(super) const fn value(self) -> &'static str {
        match self {
            Self::Text => constants_str::PG_CRUD_PG_TEXT,
            Self::Password => constants_str::PASSWORD,
            Self::Number => constants_str::VALUE_12886F9D,
            Self::Url => constants_str::VALUE_28E5EBAB,
        }
    }
}
