#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "pg type greater than test keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
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
