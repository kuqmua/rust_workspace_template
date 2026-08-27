use super::{TestExpId, panic_unexpected_result};

#[track_caller]
pub(crate) fn expect_error<T, E>(v: Result<T, E>, exp_id: impl Into<TestExpId>) -> E {
    v.err().unwrap_or_else(|| {
        panic_unexpected_result(
            constants_str::ROUTE_VALIDATORS_EXPECT_ER_ER_ID,
            constants_str::EXPECT_ERROR,
            constants_str::OK,
            exp_id,
        )
    })
}
