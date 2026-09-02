#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum AdminPageNavigation {
    OpenApi,
    Metrics,
    Profile,
    Sessions,
    Settings,
    Version,
}
