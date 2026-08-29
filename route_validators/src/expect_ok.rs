#[track_caller]
pub(crate) fn expect_ok<T, E>(
    v: Result<T, E>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
) -> T {
    v.unwrap_or_else(|_| {
        crate::panic_unexpected_result::panic_unexpected_result(
            constants_str::catalog::ROUTE_VALIDATORS_EXPECT_OK_ER_ID,
            constants_str::catalog::EXPECT_OK,
            constants_str::catalog::ERR,
            exp_id,
        )
    })
}
