pub(super) const fn rust_type_wire_kind<
    CanBeNullable,
    CanBePrimaryKey,
    FilterKind,
    PgName,
    WireKind,
>(
    spec: &crate::pg_type_spec::PgTypeSpec<
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
    *spec.get_wire_kind()
}
