#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "measurement fields are ordered by decreasing alignment to satisfy OptimalMemoryLayout"
)]
pub(crate) struct DirectGenerationMeasurement {
    maximum_wall_microseconds: u128,
    minimum_wall_microseconds: u128,
    total_wall_microseconds: u128,
    output_bytes: usize,
    output_token_trees: usize,
}
