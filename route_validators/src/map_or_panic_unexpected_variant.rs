#[track_caller]
pub(super) fn map_or_panic_unexpected_variant<R>(
    option: Option<R>,
    expectation_id: impl Into<crate::test_expectation_id::TestExpectationId>,
) -> R {
    option.unwrap_or_else(|| {
        crate::panic_unexpected_variant::panic_unexpected_variant(expectation_id)
    })
}
