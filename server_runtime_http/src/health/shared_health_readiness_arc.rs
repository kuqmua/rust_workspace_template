#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(super) struct SharedHealthReadinessArc(
    pub(super) std::sync::Arc<std::sync::atomic::AtomicBool>,
);
