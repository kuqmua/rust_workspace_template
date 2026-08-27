#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub struct PgTypeLenGreaterThanTest<T: crate::domain_types::PgType> {
    pub create: <T as crate::domain_types::PgType>::Create,
    pub variant: crate::domain_types::PgTypeGreaterThanVariant,
    pub len_greater_than: crate::domain_types::UnsignedPartOfI32,
}
