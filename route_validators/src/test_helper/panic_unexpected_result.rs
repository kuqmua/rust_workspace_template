use super::{TestExpId, TestPanicText};

#[track_caller]
pub(super) fn panic_unexpected_result(
    error_id: impl Into<TestPanicText>,
    fn_name: impl Into<TestPanicText>,
    expected: impl Into<TestPanicText>,
    exp_id: impl Into<TestExpId>,
) -> ! {
    let error_id = error_id.into();
    let fn_name = fn_name.into();
    let expected = expected.into();
    let exp_id = exp_id.into();
    panic!("{error_id} unexpected {expected} for {fn_name}, id={exp_id}");
}
