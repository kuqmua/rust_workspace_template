#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Default, Clone, Copy, PartialEq, Eq,
)]
pub enum SvcMode {
    Migrate,
    #[default]
    Serve,
}
