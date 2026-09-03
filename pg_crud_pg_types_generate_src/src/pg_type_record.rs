#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "pg type record keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    std::hash::Hash,
    serde::Serialize,
    serde::Deserialize,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[serde(try_from = "crate::pg_type_record_raw::PgTypeRecordRaw")]
pub(super) struct PgTypeRecord {
    pg_type: crate::pg_type_catalog_kind::PgTypeCatalogKind,
    is_nullable: pg_crud_macro_common::is_nullable::IsNullable,
    pg_type_pattern: crate::pg_type_pattern::PgTypePattern,
}
