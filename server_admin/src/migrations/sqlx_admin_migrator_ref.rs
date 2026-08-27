#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(crate) struct SqlxAdminMigratorRef(&'static sqlx::migrate::Migrator);
