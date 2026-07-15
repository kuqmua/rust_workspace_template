#![allow(clippy::field_scoped_visibility_modifiers)] // the private descriptor is constructed by its sibling catalog while fields remain hidden outside this generator
#[derive(Clone, Copy)]
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
            pg_name: str_constants::expr::S_1424,
            wire_kind: 32u8,
        };
        assert!(crate::sqlx::can_be_nullable(spec));
        assert!(!crate::sqlx::can_be_primary_key(spec));
        assert_eq!(crate::filter::filter_kind(spec), 7u8);
        assert_eq!(crate::catalog::pg_name(spec), "int4");
        assert_eq!(crate::schema::wire_kind(spec), 32u8);
    }
}
