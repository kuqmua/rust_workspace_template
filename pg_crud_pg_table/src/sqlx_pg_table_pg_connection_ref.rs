#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::AsMut,
    proc_macro_newtype::FromInner,
)]
pub struct SqlxPgTablePgConnectionRef<'connection_lt>(&'connection_lt mut sqlx::PgConnection);
