#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq, thiserror::Error,
)]
pub enum PgPoolConfigParseError {
    #[error("pg pool numeric configuration is invalid")]
    Parse,
    #[error("pg pool duration must be greater than zero")]
    Zero,
}
