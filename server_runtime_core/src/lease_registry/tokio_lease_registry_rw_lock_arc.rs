use super::LeaseRegistryInner;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, newtype::FromInner)]
pub(super) struct TokioLeaseRegistryRwLockArc(
    pub(super) std::sync::Arc<tokio::sync::RwLock<LeaseRegistryInner>>,
);
