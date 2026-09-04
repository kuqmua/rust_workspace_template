#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_deref_mut_inner::DerefMutInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct AxumTestHeadersMutRef<'headers_lt>(&'headers_lt mut axum::http::HeaderMap);
impl<'headers_lt> From<&'headers_lt mut crate::axum_test_headers::AxumTestHeaders>
    for AxumTestHeadersMutRef<'headers_lt>
{
    fn from(value: &'headers_lt mut crate::axum_test_headers::AxumTestHeaders) -> Self {
        Self::from(&mut **value)
    }
}
