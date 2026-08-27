#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct SqlxNotificationMigrationError(sqlx::migrate::MigrateError);
