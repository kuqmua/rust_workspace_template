#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::AsMut,
    proc_macro_newtype::FromInner,
)]
pub struct SqlxPgRelationLockConnectionRef<'connection>(&'connection mut sqlx::PgConnection);
