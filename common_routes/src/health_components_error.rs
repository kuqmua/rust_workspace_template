#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum HealthComponentsError {
    #[error("{}", constants_str::HEALTH_COMPONENTS_LENGTH_EXCEEDS_LIMIT)]
    TooMany,
}
