#[track_caller]
pub(crate) fn expect_err_variant_ref_with_status<T, E, R>(
    result: Result<T, E>,
    exp_id: impl Into<crate::test_exp_id::TestExpId>,
    option: Option<crate::axum_http_status_code::AxumHttpStatusCode>,
    map: impl FnOnce(&E) -> Option<R>,
) -> R
where
    E: crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider,
{
    let exp_id = exp_id.into();
    match option {
        Some(status_code) => {
            crate::assert_err_status_code_variant_ref::assert_err_status_code_variant_ref(
                result,
                exp_id.get(),
                status_code,
                map,
            )
        }
        None => {
            crate::expect_error_variant_ref::expect_error_variant_ref(result, exp_id.get(), map)
        }
    }
}
