#[derive(
    Debug,
    thiserror::Error,
    proc_macro_location::Location,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum ErrorUnnamedOne {
    Something(crate::error_two::ErrorTwo),
}
