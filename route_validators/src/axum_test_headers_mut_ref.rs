#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::DerefMutInner,
    proc_macro_newtype::FromInner,
)]
pub(crate) struct AxumTestHeadersMutRef<'headers_lt>(&'headers_lt mut axum::http::HeaderMap);
impl<'headers_lt> From<&'headers_lt mut crate::axum_test_headers::AxumTestHeaders>
    for AxumTestHeadersMutRef<'headers_lt>
{
    fn from(axum_test_headers: &'headers_lt mut crate::axum_test_headers::AxumTestHeaders) -> Self {
        Self::from(&mut **axum_test_headers)
    }
}
