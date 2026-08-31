#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub(crate) struct AxumTestHeadersMutRef<'headers_lt>(&'headers_lt mut axum::http::HeaderMap);
impl<'headers_lt> From<&'headers_lt mut crate::axum_test_headers::AxumTestHeaders>
    for AxumTestHeadersMutRef<'headers_lt>
{
    fn from(value: &'headers_lt mut crate::axum_test_headers::AxumTestHeaders) -> Self {
        Self::from(&mut **value)
    }
}
