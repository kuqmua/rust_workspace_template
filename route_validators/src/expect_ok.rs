#[track_caller]
pub(crate) fn expect_ok<T, E>(
    result: Result<T, E>,
    expectation_id: impl Into<crate::test_expectation_id::TestExpectationId>,
) -> T {
    result.unwrap_or_else(|_| {
        crate::panic_unexpected_result::panic_unexpected_result(
            constants_str::ROUTE_VALIDATORS_EXPECT_OK_ER_ID,
            constants_str::EXPECT_OK,
            constants_str::ERR,
            expectation_id,
        )
    })
}
