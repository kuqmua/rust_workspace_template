#[track_caller]
pub(crate) fn assert_err_status_code<T, E>(
    result: Result<T, E>,
    expectation_id: impl Into<crate::test_expectation_id::TestExpectationId>,
    axum_http_status_code: crate::axum_http_status_code::AxumHttpStatusCode,
) -> E
where
    E: crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider,
{
    crate::map_err_after_status_check::map_err_after_status_check(
        result,
        expectation_id,
        axum_http_status_code,
        |error, _| error,
    )
}
