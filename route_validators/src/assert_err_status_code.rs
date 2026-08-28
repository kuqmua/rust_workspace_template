use super::{TestExpId, map_err_after_status_check};

#[track_caller]
pub(crate) fn assert_err_status_code<T, E>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    expected: crate::AxumHttpStatusCode,
) -> E
where
    E: crate::AxumHttpStatusCodeProvider,
{
    map_err_after_status_check(v, exp_id, expected, |error, _| error)
}
