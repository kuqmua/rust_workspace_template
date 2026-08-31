pub(super) const fn pg_name<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>(
    spec: &crate::pg_type_spec::PgTypeSpec<
        CanBeNullable,
        CanBePrimaryKey,
        FilterKind,
        PgName,
        WireKind,
    >,
) -> PgName
where
    PgName: Copy,
{
    *spec.get_pg_name()
}
