#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    newtype::AsRefTarget,
    newtype::DerefTarget,
    newtype::FromInner,
)]
pub struct BytesBodyBytes(pub(super) bytes::Bytes);
