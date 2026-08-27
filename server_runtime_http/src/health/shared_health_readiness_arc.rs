#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(super) struct SharedHealthReadinessArc(
    pub(super) std::sync::Arc<std::sync::atomic::AtomicBool>,
);
