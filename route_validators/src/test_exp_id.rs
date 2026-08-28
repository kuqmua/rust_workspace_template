#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::Display, newtype::FromInner)]
pub(crate) struct TestExpId(pub(super) &'static str);
