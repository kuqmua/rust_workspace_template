#[track_caller]
pub(super) fn map_err_after_status_check<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
    expected: crate::axum_http_status_code::AxumHttpStatusCode,
    map: impl FnOnce(E, &'static str) -> R,
) -> R
where
    E: crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider,
{
    crate::map_err::map_err(
        v,
        exp_id,
        |error| {
            assert_eq!(error.axum_http_status_code(), expected);
        },
        map,
    )
}
