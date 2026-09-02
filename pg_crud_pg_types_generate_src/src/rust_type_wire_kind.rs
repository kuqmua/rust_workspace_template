pub(super) const fn rust_type_wire_kind<
    CanBeNullable,
    CanBePrimaryKey,
    FilterKind,
    PgName,
    WireKind,
>(
    pg_type_spec: &crate::pg_type_spec::PgTypeSpec<
        CanBeNullable,
        CanBePrimaryKey,
        FilterKind,
        PgName,
        WireKind,
    >,
) -> WireKind
where
    WireKind: Copy,
{
    *pg_type_spec.get_wire_kind()
}
