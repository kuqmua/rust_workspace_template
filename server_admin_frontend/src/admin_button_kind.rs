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
pub enum AdminButtonKind {
    Button,
    #[default]
    Submit,
}

impl AdminButtonKind {
    pub(super) const fn value(self) -> &'static str {
        self.as_snake_case_str()
    }
}
