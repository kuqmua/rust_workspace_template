#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(super) struct SharedHealthReadinessArc(std::sync::Arc<std::sync::atomic::AtomicBool>);
