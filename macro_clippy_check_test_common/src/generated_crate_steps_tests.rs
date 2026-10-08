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

#[cfg(test)]
#[cfg(feature = "test-utils")]
mod tests {
    #[test]
    fn test_generated_crate_step_catalog_preserves_phase_order_and_command_mapping() {
        let expected = [
            (
                crate::generated_crate_phase::GeneratedCratePhase::Formatting,
                constants_str::MACRO_CLIPPY_CARGO_FMT_ARGS.as_slice(),
            ),
            (
                crate::generated_crate_phase::GeneratedCratePhase::Compilation,
                constants_str::MACRO_CLIPPY_CARGO_CHECK_ALL_TARGETS_ALL_FEATURES_ARGS.as_slice(),
            ),
            (
                crate::generated_crate_phase::GeneratedCratePhase::Clippy,
                constants_str::MACRO_CLIPPY_CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS.as_slice(),
            ),
            (
                crate::generated_crate_phase::GeneratedCratePhase::Test,
                constants_str::MACRO_CLIPPY_CARGO_TEST_LIB_ARGS.as_slice(),
            ),
        ];
        assert!(
            crate::generated_crate_steps_tests::GENERATED_CRATE_STEPS
                .iter()
                .map(|step| (step.phase(), step.args()))
                .eq(expected)
        );
    }
}
