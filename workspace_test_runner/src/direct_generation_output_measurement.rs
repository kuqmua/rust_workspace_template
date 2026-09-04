#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(crate) struct DirectGenerationOutputMeasurement {
    output_bytes: usize,
    output_token_trees: usize,
}
