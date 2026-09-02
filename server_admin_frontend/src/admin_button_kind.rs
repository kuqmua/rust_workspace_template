#[derive(
    Debug,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    proc_macro_naming::EnumWithUnitFieldsToSnakeCaseStr,
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
