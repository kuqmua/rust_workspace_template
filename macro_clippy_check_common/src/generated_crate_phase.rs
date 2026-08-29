#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneratedCratePhase {
    Clippy,
    Compilation,
    Formatting,
    Test,
}

impl std::fmt::Display for GeneratedCratePhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clippy => f.write_str(constants_str::catalog::CLIPPY),
            Self::Compilation => f.write_str(constants_str::catalog::COMPILATION),
            Self::Formatting => f.write_str(constants_str::catalog::FORMATTING),
            Self::Test => f.write_str(constants_str::catalog::TEST_ALT_3),
        }
    }
}
