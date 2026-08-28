#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub struct ChronoUtcDateTimeRef<'value_lt>(&'value_lt chrono::DateTime<chrono::Utc>);
