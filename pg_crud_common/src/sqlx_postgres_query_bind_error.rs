#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("failed to bind PostgreSQL query parameter")]
pub struct SqlxPostgresQueryBindError {
    #[source]
    source: super::SqlxBoxDynError,
}

impl From<sqlx::error::BoxDynError> for SqlxPostgresQueryBindError {
    fn from(source: sqlx::error::BoxDynError) -> Self {
        Self {
            source: super::SqlxBoxDynError::from(source),
        }
    }
}
