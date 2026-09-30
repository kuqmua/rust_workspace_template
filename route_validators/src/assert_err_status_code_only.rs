#[track_caller]
pub(crate) fn assert_err_status_code_only<T, E>(
    result: Result<T, E>,
    expectation_id: impl Into<crate::test_expectation_id::TestExpectationId>,
    axum_http_status_code: crate::axum_http_status_code::AxumHttpStatusCode,
) where
    E: crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider,
{
    drop(crate::assert_err_status_code::assert_err_status_code(
        result,
        expectation_id,
        axum_http_status_code,
    ));
}
