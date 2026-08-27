#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct ChronoLocationDateTime(pub(super) chrono::DateTime<chrono::FixedOffset>);
