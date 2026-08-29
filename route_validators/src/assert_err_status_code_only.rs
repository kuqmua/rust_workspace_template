#[track_caller]
pub(crate) fn assert_err_status_code_only<T, E>(
    v: Result<T, E>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
    expected: crate::axum_http_status_code::AxumHttpStatusCode,
) where
    E: crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider,
{
    drop(crate::assert_err_status_code::assert_err_status_code(
        v, exp_id, expected,
    ));
}
