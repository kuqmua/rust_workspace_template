use super::{TestExpId, map_err_after_status_check};

#[track_caller]
pub(crate) fn assert_err_status_code<T, E>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    expected: crate::domain_types::AxumHttpStatusCode,
) -> E
where
    E: crate::domain_types::AxumHttpStatusCodeProvider,
{
    map_err_after_status_check(v, exp_id, expected, |error, _| error)
}
