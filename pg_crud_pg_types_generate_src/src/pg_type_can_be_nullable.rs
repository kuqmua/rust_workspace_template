pub(crate) const fn pg_type_can_be_nullable<
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
) -> CanBeNullable
where
    CanBeNullable: Copy,
{
    *spec.get_can_be_nullable()
}
