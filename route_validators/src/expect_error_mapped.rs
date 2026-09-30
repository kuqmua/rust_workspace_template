#[track_caller]
pub(crate) fn expect_error_mapped<T, E, R>(
    result: Result<T, E>,
    expectation_id: impl Into<crate::test_expectation_id::TestExpectationId>,
    map: impl FnOnce(E, &'static str) -> R,
) -> R {
    crate::map_err::map_err(result, expectation_id, |_| (), map)
}
