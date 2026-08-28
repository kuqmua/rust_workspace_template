#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::AsMut, newtype::FromInner)]
pub(in super::super) struct SqlxAdminPgConnectionRef<'connection_lt>(
    &'connection_lt mut sqlx::PgConnection,
);
