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
    std::panic::panic_any(
        constants_str::PANIC_C8FE4BAC
            .replacen(
                constants_str::PANIC_PLACEHOLDER_81766C62,
                error_id.to_string().as_str(),
                1usize,
            )
            .replacen(
                constants_str::PANIC_PLACEHOLDER_A8D54BD7,
                expected.to_string().as_str(),
                1usize,
            )
            .replacen(
                constants_str::PANIC_PLACEHOLDER_15B92257,
                fn_name.to_string().as_str(),
                1usize,
            )
            .replacen(
                constants_str::PANIC_PLACEHOLDER_D8C45567,
                exp_id.to_string().as_str(),
                1usize,
            ),
    );
}
