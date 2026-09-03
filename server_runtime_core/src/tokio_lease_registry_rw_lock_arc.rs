#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct TokioLeaseRegistryRwLockArc(
    std::sync::Arc<tokio::sync::RwLock<crate::lease_registry_inner::LeaseRegistryInner>>,
);
