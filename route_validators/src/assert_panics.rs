#[track_caller]
pub(crate) fn assert_panics(
    action: impl FnOnce() + std::panic::UnwindSafe,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
) {
    let exp_id = exp_id.into();
    let panic_res = std::panic::catch_unwind(action);
    drop(panic_res.expect_err(exp_id.get()));
}
