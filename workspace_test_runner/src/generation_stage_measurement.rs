#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "generation stage durations remain in pipeline order for measurement reporting"
)]
pub(crate) struct GenerationStageMeasurement {
    parse_microseconds: u128,
    build_microseconds: u128,
    validate_microseconds: u128,
    emit_microseconds: u128,
    output_bytes: usize,
}
