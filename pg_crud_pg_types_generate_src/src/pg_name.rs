pub(super) fn pg_name<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>(
    spec: crate::domain_types::pg_type_spec::PgTypeSpec<
        CanBeNullable,
        CanBePrimaryKey,
        FilterKind,
        PgName,
        WireKind,
    >,
) -> PgName {
    spec.pg_name
}
