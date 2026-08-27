#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::LeaseRegistryInner;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, newtype::FromInner)]
pub(super) struct TokioLeaseRegistryRwLockArc(
    pub(super) std::sync::Arc<tokio::sync::RwLock<LeaseRegistryInner>>,
);
