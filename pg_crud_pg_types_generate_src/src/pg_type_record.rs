#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    std::hash::Hash,
    serde::Serialize,
    serde::Deserialize,
    optimal_memory_layout::OptimalMemoryLayout,
)]
#[serde(try_from = "crate::pg_type_record_raw::PgTypeRecordRaw")]
pub(super) struct PgTypeRecord {
    pub(super) pg_type: crate::pg_type_catalog_kind::PgTypeCatalogKind,
    pub(super) is_nullable: pg_crud_macro_common::is_nullable::IsNullable,
    pub(super) pg_type_pattern: crate::pg_type_pattern::PgTypePattern,
}
