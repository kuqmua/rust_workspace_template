#[track_caller]
pub(super) fn map_err<T, E, R>(
    result: Result<T, E>,
    expectation_id: impl Into<crate::test_expectation_id::TestExpectationId>,
    check: impl FnOnce(&E),
    map: impl FnOnce(E, &'static str) -> R,
) -> R {
    let expectation_id = expectation_id.into();
    let error = crate::expect_error::expect_error(result, expectation_id.get());
    check(&error);
    map(error, expectation_id.get())
}
