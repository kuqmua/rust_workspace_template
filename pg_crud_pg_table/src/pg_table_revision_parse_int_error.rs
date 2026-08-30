#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum PgTableRevisionParseIntError {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}
