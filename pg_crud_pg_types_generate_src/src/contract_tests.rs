#[cfg(test)]
mod tests {
    #[test]
    fn test_all_emitter_projections_read_one_pg_type_spec() {
        let spec = crate::pg_type_spec::PgTypeSpec::new(
            true,
            false,
            7u8,
            constants_str::PG_CRUD_PG_INT4,
            32u8,
        );
        assert_eq!(crate::pg_name::pg_name(&spec), "int4");
        assert_eq!(*spec.get_filter_kind(), 7u8);
        assert_eq!(crate::rust_type_wire_kind::rust_type_wire_kind(&spec), 32u8);
        assert_eq!(crate::schema_wire_kind::schema_wire_kind(&spec), 32u8);
        assert_eq!(*spec.get_wire_kind(), 32u8);
        assert!(crate::pg_type_can_be_nullable::pg_type_can_be_nullable(
            &spec
        ));
        assert!(!*spec.get_can_be_primary_key());
    }
}
