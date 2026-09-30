#[track_caller]
pub(crate) fn expect_error_variant_ref<T, E, R>(
    result: Result<T, E>,
    expectation_id: impl Into<crate::test_expectation_id::TestExpectationId>,
    map: impl FnOnce(&E) -> Option<R>,
) -> R {
    crate::expect_error_mapped::expect_error_mapped(
        result,
        expectation_id,
        |error, mapped_expectation_id| {
            crate::expect_variant_ref::expect_variant_ref(&error, map, mapped_expectation_id)
        },
    )
}
