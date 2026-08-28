#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
#[allow(
    dead_code,
    reason = "the owned permit is held for its drop semantics while password hashing runs"
)]
pub(crate) struct TokioAdminOwnedSemaphorePermit(pub(super) tokio::sync::OwnedSemaphorePermit);
