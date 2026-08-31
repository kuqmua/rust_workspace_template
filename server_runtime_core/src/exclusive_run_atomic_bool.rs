#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::DerefInner, newtype::FromInner,
)]
pub(super) struct ExclusiveRunAtomicBool(std::sync::atomic::AtomicBool);
