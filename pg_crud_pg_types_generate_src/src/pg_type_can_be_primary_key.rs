pub(crate) fn pg_type_can_be_primary_key<
    CanBeNullable,
    CanBePrimaryKey,
    FilterKind,
    PgName,
    WireKind,
>(
    spec: crate::domain_types::pg_type_spec::PgTypeSpec<
        CanBeNullable,
        CanBePrimaryKey,
        FilterKind,
        PgName,
        WireKind,
    >,
) -> CanBePrimaryKey {
    spec.can_be_primary_key
}
