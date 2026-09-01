#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "component props and wire enum variants retain their semantic presentation order"
)]
#[derive(
    Debug,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    naming_macros::EnumWithUnitFieldsToSnakeCaseStr,
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
        self.as_snake_case_str()
    }
}
