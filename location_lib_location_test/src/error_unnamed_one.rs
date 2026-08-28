use crate::*;

#[derive(
    Debug, thiserror::Error, location::Location, optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum ErrorUnnamedOne {
    Something(ErrorTwo),
}
