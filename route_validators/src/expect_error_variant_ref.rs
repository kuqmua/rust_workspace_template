use super::{TestExpId, expect_error_mapped, expect_variant_ref};

#[track_caller]
pub(crate) fn expect_error_variant_ref<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    map: impl FnOnce(&E) -> Option<R>,
) -> R {
    expect_error_mapped(v, exp_id, |error, mapped_exp_id| {
        expect_variant_ref(&error, map, mapped_exp_id)
    })
}
