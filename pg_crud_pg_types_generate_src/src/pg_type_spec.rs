// The owner module retains lint-sensitive semantics from the original implementation.
#[derive(
    generate_accessor::Getters,
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
)]
pub(super) struct PgTypeSpec<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind> {
    can_be_nullable: CanBeNullable,
    can_be_primary_key: CanBePrimaryKey,
    filter_kind: FilterKind,
    pg_name: PgName,
    wire_kind: WireKind,
}
#[cfg(test)]
mod tests {
    #[test]
    fn pg_type_spec_keeps_storage_filter_and_wire_capabilities_together() {
        let spec = crate::pg_type_spec::PgTypeSpec::new(
            true,
            false,
            7u8,
            constants_str::PG_CRUD_PG_INT4,
            32u8,
        );
        assert!(crate::pg_type_can_be_nullable::pg_type_can_be_nullable(
            &spec
        ));
        assert!(!spec.can_be_primary_key);
        assert_eq!(spec.filter_kind, 7u8);
        assert_eq!(crate::pg_name::pg_name(&spec), "int4");
        assert_eq!(crate::schema_wire_kind::schema_wire_kind(&spec), 32u8);
    }
}
