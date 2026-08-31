#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DebugTransparent,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub struct SqlxAdminMigrateError(sqlx::migrate::MigrateError);
