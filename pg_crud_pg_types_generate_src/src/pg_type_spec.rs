// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::field_scoped_visibility_modifiers)] // the private descriptor is constructed by its sibling catalog while fields remain hidden outside this generator
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(super) struct PgTypeSpec<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind> {
    pub(super) can_be_nullable: CanBeNullable,
    pub(super) can_be_primary_key: CanBePrimaryKey,
    pub(super) filter_kind: FilterKind,
    pub(super) pg_name: PgName,
    pub(super) wire_kind: WireKind,
}
#[cfg(test)]
mod tests {
    #[test]
    fn pg_type_spec_keeps_storage_filter_and_wire_capabilities_together() {
        let spec = super::PgTypeSpec {
            can_be_nullable: true,
            can_be_primary_key: false,
            filter_kind: 7u8,
            pg_name: constants_str::PG_CRUD_PG_INT4,
            wire_kind: 32u8,
        };
        assert!(crate::domain_types::sqlx::can_be_nullable::can_be_nullable(
            spec
        ));
        assert!(!crate::domain_types::sqlx::can_be_primary_key::can_be_primary_key(spec));
        assert_eq!(crate::domain_types::filter_kind::filter_kind(spec), 7u8);
        assert_eq!(crate::domain_types::pg_name::pg_name(spec), "int4");
        assert_eq!(
            crate::domain_types::schema_wire_kind::schema_wire_kind(spec),
            32u8
        );
    }
}
