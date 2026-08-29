#[track_caller]
pub(crate) fn assert_err_status_code_variant_ref<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
    expected: crate::axum_http_status_code::AxumHttpStatusCode,
    map: impl FnOnce(&E) -> Option<R>,
) -> R
where
    E: crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider,
{
    crate::map_err_after_status_check::map_err_after_status_check(
        v,
        exp_id,
        expected,
        |error, mapped_exp_id| {
            crate::expect_variant_ref::expect_variant_ref(&error, map, mapped_exp_id)
        },
    )
}
