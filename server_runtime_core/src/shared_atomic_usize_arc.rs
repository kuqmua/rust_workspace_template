#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(super) struct SharedAtomicUsizeArc(std::sync::Arc<std::sync::atomic::AtomicUsize>);
