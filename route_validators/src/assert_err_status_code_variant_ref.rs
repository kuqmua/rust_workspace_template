#[track_caller]
pub(crate) fn assert_err_status_code_variant_ref<T, E, R>(
    result: Result<T, E>,
    expectation_id: impl Into<crate::test_expectation_id::TestExpectationId>,
    axum_http_status_code: crate::axum_http_status_code::AxumHttpStatusCode,
    map: impl FnOnce(&E) -> Option<R>,
) -> R
where
    E: crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider,
{
    crate::map_err_after_status_check::map_err_after_status_check(
        result,
        expectation_id,
        axum_http_status_code,
        |error, mapped_expectation_id| {
            crate::expect_variant_ref::expect_variant_ref(&error, map, mapped_expectation_id)
        },
    )
}
