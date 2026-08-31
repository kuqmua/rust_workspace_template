// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    generate_accessor::Getters,
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct PgTypeGreaterThanTest<T: crate::pg_type::PgType> {
    greater_than: <T as crate::pg_type::PgType>::TableType,
    create: <T as crate::pg_type::PgType>::Create,
    variant: crate::pg_type_greater_than_variant::PgTypeGreaterThanVariant,
}
impl<T: crate::pg_type::PgType> PgTypeGreaterThanTest<T> {
    pub fn into_parts(
        self,
    ) -> (
        <T as crate::pg_type::PgType>::TableType,
        <T as crate::pg_type::PgType>::Create,
        crate::pg_type_greater_than_variant::PgTypeGreaterThanVariant,
    ) {
        (self.greater_than, self.create, self.variant)
    }
}
