// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_getters::Getters,
)]
pub struct PgTypeLenGreaterThanTest<T: crate::pg_type::PgType> {
    create: <T as crate::pg_type::PgType>::Create,
    variant: crate::pg_type_greater_than_variant::PgTypeGreaterThanVariant,
    len_greater_than: crate::unsigned_part_of_i32::UnsignedPartOfI32,
}
