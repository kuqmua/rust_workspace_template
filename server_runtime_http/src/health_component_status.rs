#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum HealthComponentStatus {
    Error,
    Ok,
}
