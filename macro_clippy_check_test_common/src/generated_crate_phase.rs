#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    naming_macros::EnumWithUnitFieldsToSnakeCaseStr,
)]
pub(crate) enum GeneratedCratePhase {
    Clippy,
    Compilation,
    Formatting,
    Test,
}

impl std::fmt::Display for GeneratedCratePhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_snake_case_str())
    }
}
