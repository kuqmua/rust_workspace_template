#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsRefTarget, newtype::FromInner,
)]
pub(crate) struct JsonwebtokenAdminDecodingKeys(pub(crate) Vec<jsonwebtoken::DecodingKey>);
