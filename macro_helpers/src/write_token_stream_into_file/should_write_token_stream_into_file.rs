#[derive(Debug, Copy, Clone, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout)]
pub enum ShouldWriteTokenStreamIntoFile {
    False,
    True,
}
