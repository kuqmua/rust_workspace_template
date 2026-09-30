#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum ServiceMode {
    Migrate,
    #[default]
    Serve,
}
