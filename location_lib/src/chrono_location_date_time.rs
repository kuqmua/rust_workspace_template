#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(super) struct ChronoLocationDateTime(chrono::DateTime<chrono::FixedOffset>);
