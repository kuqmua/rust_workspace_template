#[track_caller]
pub(crate) fn expect_variant<T, R>(
    t: T,
    map: impl FnOnce(T) -> Option<R>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
) -> R {
    crate::map_or_panic_unexpected_variant::map_or_panic_unexpected_variant(map(t), exp_id)
}
