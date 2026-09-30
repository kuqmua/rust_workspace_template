#[track_caller]
pub(crate) fn expect_error<T, E>(
    result: Result<T, E>,
    expectation_id: impl Into<crate::test_expectation_id::TestExpectationId>,
) -> E {
    result.err().unwrap_or_else(|| {
        crate::panic_unexpected_result::panic_unexpected_result(
            constants_str::ROUTE_VALIDATORS_EXPECT_ER_ER_ID,
            constants_str::EXPECT_ERROR,
            constants_str::OK,
            expectation_id,
        )
    })
}
