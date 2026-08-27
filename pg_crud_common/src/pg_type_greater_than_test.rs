// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, optimal_memory_layout::OptimalMemoryLayout)]
pub struct PgTypeGreaterThanTest<T: crate::domain_types::PgType> {
    pub greater_than: <T as crate::domain_types::PgType>::TableType,
    pub create: <T as crate::domain_types::PgType>::Create,
    pub variant: crate::domain_types::PgTypeGreaterThanVariant,
}
