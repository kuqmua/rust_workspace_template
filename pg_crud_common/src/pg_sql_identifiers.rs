#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::FromInner,
)]
pub struct PgSqlIdentifiers(pub(super) Vec<crate::domain_types::SqlIdentifier>);
