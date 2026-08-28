#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", constants_str::HEALTH_COMPONENTS_LENGTH_EXCEEDS_LIMIT)]
pub struct HealthComponentsError;
