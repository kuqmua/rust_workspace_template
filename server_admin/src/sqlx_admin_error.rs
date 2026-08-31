#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DebugTransparent,
    thiserror::Error,
    newtype::FromInner,
    newtype::IntoInner,
)]
#[error(transparent)]
#[derive(generate_accessor::Getters)]
pub struct SqlxAdminError(sqlx::Error);
