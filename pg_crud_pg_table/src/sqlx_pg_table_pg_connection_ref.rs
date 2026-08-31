#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsMut, newtype::FromInner)]
pub struct SqlxPgTablePgConnectionRef<'connection_lt>(&'connection_lt mut sqlx::PgConnection);
