#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(super) struct TokioLeaseRegistryRwLockArc(
    std::sync::Arc<tokio::sync::RwLock<crate::lease_registry_inner::LeaseRegistryInner>>,
);
