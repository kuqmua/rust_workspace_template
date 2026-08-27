#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsMut, newtype::FromInner)]
pub struct SqlxPgRelationLockConnectionRef<'connection>(&'connection mut sqlx::PgConnection);
