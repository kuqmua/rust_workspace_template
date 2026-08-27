use super::{TestExpId, map_err};

#[track_caller]
pub(super) fn map_err_after_status_check<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    expected: crate::domain_types::AxumHttpStatusCode,
    map: impl FnOnce(E, &'static str) -> R,
) -> R
where
    E: crate::domain_types::AxumHttpStatusCodeProvider,
{
    map_err(
        v,
        exp_id,
        |error| {
            assert_eq!(error.axum_http_status_code(), expected);
        },
        map,
    )
}
