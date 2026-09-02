#[track_caller]
pub(crate) fn expect_error_variant_ref<T, E, R>(
    result: Result<T, E>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
    map: impl FnOnce(&E) -> Option<R>,
) -> R {
    crate::expect_error_mapped::expect_error_mapped(result, exp_id, |error, mapped_exp_id| {
        crate::expect_variant_ref::expect_variant_ref(&error, map, mapped_exp_id)
    })
}
