#[cfg(test)]
mod tests {
    #[test]
    fn all_emitter_projections_read_one_pg_type_spec() {
        let spec = crate::model::PgTypeSpec {
            can_be_nl: true,
            can_be_pk: false,
            flt_kind: 7u8,
            pg_name: "int4",
            wire_kind: 32u8,
        };
        assert_eq!(crate::catalog::pg_name(spec), "int4");
        assert_eq!(crate::filter::flt_kind(spec), 7u8);
        assert_eq!(crate::rust_type::wire_kind(spec), 32u8);
        assert_eq!(crate::schema::wire_kind(spec), 32u8);
        assert_eq!(crate::serde::wire_kind(spec), 32u8);
        assert!(crate::sqlx::can_be_nl(spec));
        assert!(!crate::sqlx::can_be_pk(spec));
    }
}
