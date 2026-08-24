#[cfg(feature = "test-utils")]
const GENERATED_CRATE_STEPS: [GeneratedCrateStep; 4] = [
    GeneratedCrateStep::new(
        &constants_str::MACRO_CLIPPY_CARGO_FMT_ARGS,
        GeneratedCratePhase::Formatting,
    ),
    GeneratedCrateStep::new(
        &constants_str::MACRO_CLIPPY_CARGO_CHECK_ALL_TARGETS_ALL_FEATURES_ARGS,
        GeneratedCratePhase::Compilation,
    ),
    GeneratedCrateStep::new(
        &constants_str::MACRO_CLIPPY_CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS,
        GeneratedCratePhase::Clippy,
    ),
    GeneratedCrateStep::new(
        &constants_str::MACRO_CLIPPY_CARGO_TEST_LIB_ARGS,
        GeneratedCratePhase::Test,
    ),
];

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[cfg(feature = "test-utils")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneratedCratePhase {
    Clippy,
    Compilation,
    Formatting,
    Test,
}

#[cfg(feature = "test-utils")]
impl std::fmt::Display for GeneratedCratePhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clippy => f.write_str(constants_str::CLIPPY),
            Self::Compilation => f.write_str(constants_str::COMPILATION),
            Self::Formatting => f.write_str(constants_str::FORMATTING),
            Self::Test => f.write_str(constants_str::TEST_ALT_3),
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[cfg(feature = "test-utils")]
pub(crate) struct GeneratedCrateStep {
    args: &'static [&'static str],
    phase: GeneratedCratePhase,
}

#[cfg(feature = "test-utils")]
impl GeneratedCrateStep {
    pub(crate) const fn args(&self) -> &'static [&'static str] {
        self.args
    }

    const fn new(args: &'static [&'static str], phase: GeneratedCratePhase) -> Self {
        Self { args, phase }
    }

    pub(crate) const fn phase(&self) -> GeneratedCratePhase {
        self.phase
    }
}

#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the crate-root Drop adapter owns filesystem cleanup for this private domain guard"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[cfg(feature = "test-utils")]
pub(crate) struct RemoveDirOnDrop {
    pub(crate) path: std::path::PathBuf,
}

#[cfg(feature = "test-utils")]
#[allow(
    clippy::single_call_fn,
    reason = "the accessor keeps the reviewed generated-crate phase catalog immutable"
)]
pub(crate) const fn generated_crate_steps() -> &'static [GeneratedCrateStep] {
    &GENERATED_CRATE_STEPS
}
