use super::{TestExpId, assert_err_status_code_variant_ref, expect_error_variant_ref};

#[track_caller]
pub(crate) fn expect_err_variant_ref_with_status<T, E, R>(
    v: Result<T, E>,
    exp_id: impl Into<TestExpId>,
    expected: Option<crate::AxumHttpStatusCode>,
    map: impl FnOnce(&E) -> Option<R>,
) -> R
where
    E: crate::AxumHttpStatusCodeProvider,
{
    let exp_id = exp_id.into();
    match expected {
        Some(status_code) => assert_err_status_code_variant_ref(v, exp_id.0, status_code, map),
        None => expect_error_variant_ref(v, exp_id.0, map),
    }
}
