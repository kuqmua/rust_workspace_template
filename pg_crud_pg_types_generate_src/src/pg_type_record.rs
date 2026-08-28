#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::*;

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
#[serde(try_from = "PgTypeRecordRaw")]
pub(super) struct PgTypeRecord {
    pub(super) pg_type: PgType,
    pub(super) is_nullable: pg_crud_macro_common::domain_types::IsNullable,
    pub(super) pg_type_pattern: PgTypePattern,
}
