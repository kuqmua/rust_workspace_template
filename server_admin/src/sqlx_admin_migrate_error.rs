#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct SqlxAdminMigrateError(sqlx::migrate::MigrateError);
