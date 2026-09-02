#[track_caller]
pub(super) fn map_or_panic_unexpected_variant<R>(
    option: Option<R>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
) -> R {
    option.unwrap_or_else(|| crate::panic_unexpected_variant::panic_unexpected_variant(exp_id))
}
