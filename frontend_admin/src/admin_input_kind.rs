#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "component props and wire enum variants retain their semantic presentation order"
)]
#[derive(
    Debug,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    proc_macro_naming_enum_with_unit_fields_to_snake_case_str::EnumWithUnitFieldsToSnakeCaseStr,
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
