#[track_caller]
pub(super) fn panic_unexpected_result(
    error_id: impl Into<crate::test_panic_text::TestPanicText>,
    fn_name: impl Into<crate::test_panic_text::TestPanicText>,
    expected: impl Into<crate::test_panic_text::TestPanicText>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
) -> ! {
    let error_id = error_id.into();
    let fn_name = fn_name.into();
    let expected = expected.into();
    let exp_id = exp_id.into();
    panic!("{error_id} unexpected {expected} for {fn_name}, id={exp_id}");
}
