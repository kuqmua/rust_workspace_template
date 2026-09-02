#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq,
)]
pub enum AdminPageCapability {
    Always,
    Swagger,
}
