// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    Copy,
    utoipa::ToSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
)] //todo check somehow what its eq to std::time::Duration
pub struct StdTimeDuration {
    secs: crate::std_time_duration_secs::StdTimeDurationSecs,
    nanos: crate::std_time_duration_nanos::StdTimeDurationNanos,
}
