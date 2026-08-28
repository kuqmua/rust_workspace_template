pub(super) fn pg_type_filter_kind<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>(
    spec: crate::domain_types::pg_type_spec::PgTypeSpec<
        CanBeNullable,
        CanBePrimaryKey,
        FilterKind,
        PgName,
        WireKind,
    >,
) -> FilterKind {
    spec.filter_kind
}
