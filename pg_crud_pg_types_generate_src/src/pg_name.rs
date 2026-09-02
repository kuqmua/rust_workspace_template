pub(super) const fn pg_name<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>(
    pg_type_spec: &crate::pg_type_spec::PgTypeSpec<
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
    *pg_type_spec.get_pg_name()
}
