#[cfg(test)]
mod tests {
    #[test]
    fn all_emitter_projections_read_one_pg_type_spec() {
        let spec = crate::model::PgTypeSpec {
            can_be_nullable: true,
            can_be_primary_key: false,
            filter_kind: 7u8,
            pg_name: "int4",
            wire_kind: 32u8,
        };
        assert_eq!(crate::catalog::pg_name(spec), "int4");
        assert_eq!(crate::filter::filter_kind(spec), 7u8);
        assert_eq!(crate::rust_type::wire_kind(spec), 32u8);
        assert_eq!(crate::schema::wire_kind(spec), 32u8);
        assert_eq!(crate::serde::wire_kind(spec), 32u8);
        assert!(crate::sqlx::can_be_nullable(spec));
        assert!(!crate::sqlx::can_be_primary_key(spec));
    }
}
