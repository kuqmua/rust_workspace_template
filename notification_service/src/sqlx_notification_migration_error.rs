#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::Display,
)]
pub(crate) struct SqlxNotificationMigrationError(sqlx::migrate::MigrateError);
