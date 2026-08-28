use super::{TestExpId, panic_unexpected_result};

#[track_caller]
pub(crate) fn expect_ok<T, E>(v: Result<T, E>, exp_id: impl Into<TestExpId>) -> T {
    v.unwrap_or_else(|_| {
        panic_unexpected_result(
            constants_str::ROUTE_VALIDATORS_EXPECT_OK_ER_ID,
            constants_str::EXPECT_OK,
            constants_str::ERR,
            exp_id,
        )
    })
}
