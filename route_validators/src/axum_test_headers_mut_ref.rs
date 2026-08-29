// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::field_scoped_visibility_modifiers)] // split wrapper representation is consumed only by its parent test-helper facade

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(crate) struct AxumTestHeadersMutRef<'headers_lt>(
    pub(super) &'headers_lt mut axum::http::HeaderMap,
);
impl<'headers_lt> From<&'headers_lt mut crate::axum_test_headers::AxumTestHeaders>
    for AxumTestHeadersMutRef<'headers_lt>
{
    fn from(value: &'headers_lt mut crate::axum_test_headers::AxumTestHeaders) -> Self {
        Self(&mut value.0)
    }
}
