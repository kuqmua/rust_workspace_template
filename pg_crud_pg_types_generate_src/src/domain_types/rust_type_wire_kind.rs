pub(super) fn wire_kind<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>(
    spec: crate::domain_types::model::PgTypeSpec<
        CanBeNullable,
        CanBePrimaryKey,
        FilterKind,
        PgName,
        WireKind,
    >,
) -> WireKind {
    spec.wire_kind
}
