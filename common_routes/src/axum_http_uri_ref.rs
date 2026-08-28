#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(crate) struct AxumHttpUriRef<'uri_lt>(pub(super) &'uri_lt axum::http::Uri);
