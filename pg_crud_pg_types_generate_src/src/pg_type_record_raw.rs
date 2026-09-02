// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde::Deserialize, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct PgTypeRecordRaw {
    pg_type: crate::pg_type_catalog_kind::PgTypeCatalogKind,
    is_nullable: pg_crud_macro_common::is_nullable::IsNullable,
    pg_type_pattern: crate::pg_type_pattern::PgTypePattern,
}
impl TryFrom<PgTypeRecordRaw> for crate::pg_type_record::PgTypeRecord {
    type Error = String;
    fn try_from(value: PgTypeRecordRaw) -> Result<Self, Self::Error> {
        let cant_supp_nullable_variants_message = constants_str::CANT_SUPPORT_NULLABLE_VARIANTS;
        match &value.pg_type.pg_type_can_be_nullable() {
            crate::can_be_nullable::CanBeNullable::False => {
                if matches!(
                    &value.is_nullable,
                    pg_crud_macro_common::is_nullable::IsNullable::True
                ) {
                    return Err(format!("{cant_supp_nullable_variants_message}{value:#?}"));
                }
                Ok(Self::new(
                    value.pg_type,
                    value.is_nullable,
                    value.pg_type_pattern,
                ))
            }
            crate::can_be_nullable::CanBeNullable::True => Ok(Self::new(
                value.pg_type,
                value.is_nullable,
                value.pg_type_pattern,
            )),
        }
    }
}
