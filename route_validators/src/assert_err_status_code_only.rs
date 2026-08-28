use super::{TestExpId, assert_err_status_code};

#[track_caller]
pub(crate) fn assert_err_status_code_only<T, E>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    expected: crate::AxumHttpStatusCode,
) where
    E: crate::AxumHttpStatusCodeProvider,
{
    drop(assert_err_status_code(v, exp_id, expected));
}
