#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_as_mut::AsMut,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct SqlxPgTablePgConnectionRef<'connection_lt>(&'connection_lt mut sqlx::PgConnection);
