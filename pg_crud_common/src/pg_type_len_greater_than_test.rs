// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub struct PgTypeLenGreaterThanTest<T: crate::pg_type::PgType> {
    pub create: <T as crate::pg_type::PgType>::Create,
    pub variant: crate::pg_type_greater_than_variant::PgTypeGreaterThanVariant,
    pub len_greater_than: crate::unsigned_part_of_i32::UnsignedPartOfI32,
}
