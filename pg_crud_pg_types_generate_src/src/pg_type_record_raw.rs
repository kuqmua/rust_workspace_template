#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "pg type record raw keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(
    proc_macro_getters::Getters,
    Debug,
    serde::Deserialize,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[getters(bare)]
pub(super) struct PgTypeRecordRaw {
    #[getters(copy)]
    pg_type: crate::pg_type_catalog_kind::PgTypeCatalogKind,
    #[getters(copy)]
    is_nullable: pg_crud_macro_common::is_nullable::IsNullable,
    #[getters(copy)]
    pg_type_pattern: crate::pg_type_pattern::PgTypePattern,
}
