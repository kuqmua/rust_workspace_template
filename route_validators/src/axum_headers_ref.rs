#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
)]
pub struct AxumHeadersRef<'headers_lt>(&'headers_lt axum::http::HeaderMap);

impl<'headers_lt> AxumHeadersRef<'headers_lt> {
    pub(crate) fn header(
        self,
        name: impl axum::http::header::AsHeaderName,
    ) -> Option<&'headers_lt axum::http::HeaderValue> {
        self.0.get(name)
    }
}

#[cfg(test)]
impl<'headers_lt> From<&'headers_lt crate::axum_test_headers::AxumTestHeaders>
    for AxumHeadersRef<'headers_lt>
{
    fn from(value: &'headers_lt crate::axum_test_headers::AxumTestHeaders) -> Self {
        Self(value.as_ref())
    }
}
