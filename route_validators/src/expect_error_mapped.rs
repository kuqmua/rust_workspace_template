#[track_caller]
pub(crate) fn expect_error_mapped<T, E, R>(
    result: Result<T, E>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
    map: impl FnOnce(E, &'static str) -> R,
) -> R {
    crate::map_err::map_err(result, exp_id, |_| (), map)
}
