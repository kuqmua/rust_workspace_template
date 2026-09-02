#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub struct IsPrimaryKey(bool);

impl From<pg_crud_common::pg_is_primary_key::PgIsPrimaryKey> for IsPrimaryKey {
    fn from(pg_is_primary_key: pg_crud_common::pg_is_primary_key::PgIsPrimaryKey) -> Self {
        Self::from(bool::from(pg_is_primary_key))
    }
}
