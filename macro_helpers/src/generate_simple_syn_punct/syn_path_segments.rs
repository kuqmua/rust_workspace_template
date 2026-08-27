#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    newtype::ToTokens,
)]
pub struct SynPathSegments(
    pub(super) syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>,
);
