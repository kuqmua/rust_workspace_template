use super::{TestExpId, expect_variant_ref, map_err_after_status_check};

#[track_caller]
pub(crate) fn assert_err_status_code_variant_ref<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    expected: crate::AxumHttpStatusCode,
    map: impl FnOnce(&E) -> Option<R>,
) -> R
where
    E: crate::AxumHttpStatusCodeProvider,
{
    map_err_after_status_check(v, exp_id, expected, |error, mapped_exp_id| {
        expect_variant_ref(&error, map, mapped_exp_id)
    })
}
