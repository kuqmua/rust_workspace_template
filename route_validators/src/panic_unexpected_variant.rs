#[track_caller]
pub(crate) fn panic_unexpected_variant(exp_id: impl Into<crate::test_exp_id::TestExpId>) -> ! {
    let exp_id = exp_id.into();
    std::panic::panic_any(constants_str::PANIC_4FE6F2E6.replacen(
        constants_str::PANIC_PLACEHOLDER_D8C45567,
        exp_id.to_string().as_str(),
        1usize,
    ));
}
