#[derive(
    Debug, Copy, Clone, serde::Deserialize, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum ShouldWriteTokenStreamIntoFile {
    False,
    True,
}
