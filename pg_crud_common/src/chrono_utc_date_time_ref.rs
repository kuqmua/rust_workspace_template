#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub struct ChronoUtcDateTimeRef<'value_lt>(&'value_lt chrono::DateTime<chrono::Utc>);
