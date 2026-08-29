pub(super) fn pg_name<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>(
    spec: crate::pg_type_spec::PgTypeSpec<
        CanBeNullable,
        CanBePrimaryKey,
        FilterKind,
        PgName,
        WireKind,
    >,
) -> PgName {
    spec.pg_name
}
