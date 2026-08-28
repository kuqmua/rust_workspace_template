#[allow(
    clippy::single_call_fn,
    reason = "SQLx primary-key projection is isolated from descriptor consumers"
)]
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
