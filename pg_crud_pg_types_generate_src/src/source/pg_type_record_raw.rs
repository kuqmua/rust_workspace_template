#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::*;

// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct PgTypeRecordRaw {
    pub(super) pg_type: PgType,
    pub(super) is_nullable: pg_crud_macro_common::domain_types::IsNullable,
    pub(super) pg_type_pattern: PgTypePattern,
}
impl TryFrom<PgTypeRecordRaw> for PgTypeRecord {
    type Error = String;
    fn try_from(v: PgTypeRecordRaw) -> Result<Self, Self::Error> {
        let cant_supp_nullable_variants_message = constants_str::CANT_SUPPORT_NULLABLE_VARIANTS;
        match &v.pg_type.can_be_nullable() {
            CanBeNullable::False => {
                if matches!(
                    &v.is_nullable,
                    pg_crud_macro_common::domain_types::IsNullable::True
                ) {
                    return Err(format!("{cant_supp_nullable_variants_message}{v:#?}"));
                }
                Ok(Self {
                    pg_type: v.pg_type,
                    is_nullable: v.is_nullable,
                    pg_type_pattern: v.pg_type_pattern,
                })
            }
            CanBeNullable::True => Ok(Self {
                pg_type: v.pg_type,
                is_nullable: v.is_nullable,
                pg_type_pattern: v.pg_type_pattern,
            }),
        }
    }
}
