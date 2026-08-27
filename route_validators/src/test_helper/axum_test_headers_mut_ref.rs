use super::AxumTestHeaders;

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(crate) struct AxumTestHeadersMutRef<'headers_lt>(
    pub(super) &'headers_lt mut axum::http::HeaderMap,
);
impl<'headers_lt> From<&'headers_lt mut AxumTestHeaders> for AxumTestHeadersMutRef<'headers_lt> {
    fn from(value: &'headers_lt mut AxumTestHeaders) -> Self {
        Self(&mut value.0)
    }
}
