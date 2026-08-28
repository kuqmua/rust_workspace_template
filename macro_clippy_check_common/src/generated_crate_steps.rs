#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the generated execution steps precede their supporting owner modules"
)]
#[cfg(feature = "test-utils")]
pub(crate) const GENERATED_CRATE_STEPS: [GeneratedCrateStep; 4] = [
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

#[cfg(feature = "test-utils")]
pub(crate) use crate::generated_crate_phase::GeneratedCratePhase;
#[cfg(feature = "test-utils")]
pub(crate) use crate::generated_crate_step::GeneratedCrateStep;
#[cfg(feature = "test-utils")]
pub(crate) use crate::remove_dir_on_drop::RemoveDirOnDrop;
