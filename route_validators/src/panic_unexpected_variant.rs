#[track_caller]
pub(crate) fn panic_unexpected_variant(
    expectation_id: impl Into<crate::test_expectation_id::TestExpectationId>,
) -> ! {
    let expectation_id = expectation_id.into();
    std::panic::panic_any(constants_str::PANIC_4FE6F2E6.replacen(
        constants_str::PANIC_PLACEHOLDER_D8C45567,
        expectation_id.to_string().as_str(),
        1usize,
    ));
}
