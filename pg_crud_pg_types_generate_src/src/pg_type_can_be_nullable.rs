pub(crate) const fn pg_type_can_be_nullable<
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
) -> CanBeNullable
where
    CanBeNullable: Copy,
{
    *pg_type_spec.get_can_be_nullable()
}
