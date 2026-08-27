#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default, newtype::FromInner)]
pub(super) struct GenerationAtomicU64(pub(super) std::sync::atomic::AtomicU64);
