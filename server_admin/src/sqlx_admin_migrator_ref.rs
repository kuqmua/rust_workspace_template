#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
    proc_macro_getters::Getters,
)]
pub(crate) struct SqlxAdminMigratorRef(&'static sqlx::migrate::Migrator);
