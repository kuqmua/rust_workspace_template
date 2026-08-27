#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub(crate) struct SqlxBoxDynError(#[source] sqlx::error::BoxDynError);
