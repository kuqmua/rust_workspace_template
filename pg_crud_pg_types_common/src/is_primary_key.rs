#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct IsPrimaryKey(bool);

impl From<pg_crud_common::is_primary_key::IsPrimaryKey> for IsPrimaryKey {
    fn from(value: pg_crud_common::is_primary_key::IsPrimaryKey) -> Self {
        Self::from(bool::from(value))
    }
}
