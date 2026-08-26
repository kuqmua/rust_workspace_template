#[cfg(test)]
mod tests {
    #[test]
    fn all_emitter_projections_read_one_pg_type_spec() {
        let spec = crate::domain_types::pg_type_spec::PgTypeSpec {
            can_be_nullable: true,
            can_be_primary_key: false,
            filter_kind: 7u8,
            pg_name: constants_str::PG_CRUD_PG_INT4,
            wire_kind: 32u8,
        };
        assert_eq!(crate::domain_types::pg_name::pg_name(spec), "int4");
        assert_eq!(crate::domain_types::filter_kind::filter_kind(spec), 7u8);
        assert_eq!(
            crate::domain_types::rust_type_wire_kind::rust_type_wire_kind(spec),
            32u8
        );
        assert_eq!(
            crate::domain_types::schema_wire_kind::schema_wire_kind(spec),
            32u8
        );
        assert_eq!(
            crate::domain_types::serde_wire_kind::serde_wire_kind(spec),
            32u8
        );
        assert!(crate::domain_types::sqlx::can_be_nullable::can_be_nullable(
            spec
        ));
        assert!(!crate::domain_types::sqlx::can_be_primary_key::can_be_primary_key(spec));
    }
}
