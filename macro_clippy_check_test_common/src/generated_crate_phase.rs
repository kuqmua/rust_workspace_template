#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_naming_enum_with_unit_fields_to_snake_case_str::EnumWithUnitFieldsToSnakeCaseStr,
)]
pub(crate) enum GeneratedCratePhase {
    Clippy,
    Compilation,
    Formatting,
    Test,
}

impl std::fmt::Display for GeneratedCratePhase {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_snake_case_str())
    }
}
