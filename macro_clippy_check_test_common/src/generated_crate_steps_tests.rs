#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the generated execution steps precede their supporting owner modules"
)]
#[cfg(feature = "test-utils")]
pub(crate) const GENERATED_CRATE_STEPS: [crate::generated_crate_step::GeneratedCrateStep; 4] = [
    crate::generated_crate_step::GeneratedCrateStep::new(
        &constants_str::MACRO_CLIPPY_CARGO_FMT_ARGS,
        crate::generated_crate_phase::GeneratedCratePhase::Formatting,
    ),
    crate::generated_crate_step::GeneratedCrateStep::new(
        &constants_str::MACRO_CLIPPY_CARGO_CHECK_ALL_TARGETS_ALL_FEATURES_ARGS,
        crate::generated_crate_phase::GeneratedCratePhase::Compilation,
    ),
    crate::generated_crate_step::GeneratedCrateStep::new(
        &constants_str::MACRO_CLIPPY_CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS,
        crate::generated_crate_phase::GeneratedCratePhase::Clippy,
    ),
    crate::generated_crate_step::GeneratedCrateStep::new(
        &constants_str::MACRO_CLIPPY_CARGO_TEST_LIB_ARGS,
        crate::generated_crate_phase::GeneratedCratePhase::Test,
    ),
];
