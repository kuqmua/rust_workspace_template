// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, optimal_memory_layout::OptimalMemoryLayout)]
pub struct PgTypeGreaterThanTest<T: crate::pg_type::PgType> {
    pub greater_than: <T as crate::pg_type::PgType>::TableType,
    pub create: <T as crate::pg_type::PgType>::Create,
    pub variant: crate::pg_type_greater_than_variant::PgTypeGreaterThanVariant,
}
