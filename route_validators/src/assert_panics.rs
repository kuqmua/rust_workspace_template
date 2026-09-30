#[track_caller]
pub(crate) fn assert_panics(
    action: impl FnOnce() + std::panic::UnwindSafe,
    expectation_id: impl Into<crate::test_expectation_id::TestExpectationId>,
) {
    let expectation_id = expectation_id.into();
    let panic_result = std::panic::catch_unwind(action);
    drop(panic_result.expect_err(expectation_id.get()));
}
