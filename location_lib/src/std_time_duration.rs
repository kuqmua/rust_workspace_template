#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "std time duration keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(
    Debug,
    Clone,
    Copy,
    utoipa::ToSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
)]
pub struct StdTimeDuration {
    secs: crate::std_time_duration_secs::StdTimeDurationSecs,
    nanos: crate::std_time_duration_nanos::StdTimeDurationNanos,
}
