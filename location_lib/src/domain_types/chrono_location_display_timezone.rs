#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct ChronoLocationDisplayTimezone(pub(super) chrono::FixedOffset);
