#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("failed to bind PostgreSQL query parameter")]
pub struct SqlxPostgresQueryBindError {
    #[source]
    source: crate::sqlx_box_dyn_error::SqlxBoxDynError,
}

impl From<sqlx::error::BoxDynError> for SqlxPostgresQueryBindError {
    fn from(box_dyn_error: sqlx::error::BoxDynError) -> Self {
        Self {
            source: crate::sqlx_box_dyn_error::SqlxBoxDynError::from(box_dyn_error),
        }
    }
}
