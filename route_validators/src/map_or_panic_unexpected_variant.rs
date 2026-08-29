#[track_caller]
pub(super) fn map_or_panic_unexpected_variant<R>(
    map_res: Option<R>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
) -> R {
    map_res.unwrap_or_else(|| crate::panic_unexpected_variant::panic_unexpected_variant(exp_id))
}
