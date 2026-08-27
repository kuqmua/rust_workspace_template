#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(super) struct SharedAtomicUsizeArc(pub(super) std::sync::Arc<std::sync::atomic::AtomicUsize>);
