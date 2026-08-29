#[derive(
    Debug, thiserror::Error, location::Location, optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum ErrorUnnamedOne {
    Something(crate::error_two::ErrorTwo),
}
