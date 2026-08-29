#[track_caller]
pub(crate) fn expect_error<T, E>(
    v: Result<T, E>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
) -> E {
    v.err().unwrap_or_else(|| {
        crate::panic_unexpected_result::panic_unexpected_result(
            constants_str::catalog::ROUTE_VALIDATORS_EXPECT_ER_ER_ID,
            constants_str::catalog::EXPECT_ERROR,
            constants_str::catalog::OK,
            exp_id,
        )
    })
}
