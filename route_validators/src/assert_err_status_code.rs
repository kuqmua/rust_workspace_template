#[track_caller]
pub(crate) fn assert_err_status_code<T, E>(
    v: Result<T, E>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
    expected: crate::axum_http_status_code::AxumHttpStatusCode,
) -> E
where
    E: crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider,
{
    crate::map_err_after_status_check::map_err_after_status_check(
        v,
        exp_id,
        expected,
        |error, _| error,
    )
}
