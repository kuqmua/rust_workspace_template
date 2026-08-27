#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct TokioLeaseInstant(pub(super) tokio::time::Instant);
