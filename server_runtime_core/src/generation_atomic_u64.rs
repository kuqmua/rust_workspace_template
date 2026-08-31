#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Default,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(super) struct GenerationAtomicU64(std::sync::atomic::AtomicU64);
