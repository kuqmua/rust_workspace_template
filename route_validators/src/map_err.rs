#[track_caller]
pub(super) fn map_err<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
    check: impl FnOnce(&E),
    map: impl FnOnce(E, &'static str) -> R,
) -> R {
    let exp_id = exp_id.into();
    let error = crate::expect_error::expect_error(v, exp_id.get());
    check(&error);
    map(error, exp_id.get())
}
