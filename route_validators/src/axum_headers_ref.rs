#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub struct AxumHeadersRef<'headers_lt>(pub(super) &'headers_lt axum::http::HeaderMap);

#[cfg(test)]
impl<'headers_lt> From<&'headers_lt crate::axum_test_headers::AxumTestHeaders>
    for AxumHeadersRef<'headers_lt>
{
    fn from(value: &'headers_lt crate::axum_test_headers::AxumTestHeaders) -> Self {
        Self(value.as_ref())
    }
}
