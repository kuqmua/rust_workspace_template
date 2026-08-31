// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    Copy,
    utoipa::ToSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor::Getters,
)] //todo check somehow what its eq to std::time::Duration
pub struct StdTimeDuration {
    secs: crate::std_time_duration_secs::StdTimeDurationSecs,
    nanos: crate::std_time_duration_nanos::StdTimeDurationNanos,
}
