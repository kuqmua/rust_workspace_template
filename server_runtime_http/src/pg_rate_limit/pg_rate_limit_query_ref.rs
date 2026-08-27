#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct PgRateLimitQueryRef(pub(super) &'static str);
