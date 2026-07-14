#![allow(clippy::field_scoped_visibility_modifiers)] // the private descriptor is constructed by its sibling catalog while fields remain hidden outside this generator
#[derive(Clone, Copy)]
pub(super) struct PgTypeSpec<CanBeNl, CanBePk, FltKind, PgName, WireKind> {
    pub(super) can_be_nl: CanBeNl,
    pub(super) can_be_pk: CanBePk,
    pub(super) flt_kind: FltKind,
    pub(super) pg_name: PgName,
    pub(super) wire_kind: WireKind,
}
#[cfg(test)]
mod tests {
    #[test]
    fn pg_type_spec_keeps_storage_filter_and_wire_capabilities_together() {
        let spec = super::PgTypeSpec {
            can_be_nl: true,
            can_be_pk: false,
            flt_kind: 7u8,
            pg_name: "int4",
            wire_kind: 32u8,
        };
        assert!(crate::sqlx::can_be_nl(spec));
        assert!(!crate::sqlx::can_be_pk(spec));
        assert_eq!(crate::filter::flt_kind(spec), 7u8);
        assert_eq!(crate::catalog::pg_name(spec), "int4");
        assert_eq!(crate::schema::wire_kind(spec), 32u8);
    }
}
