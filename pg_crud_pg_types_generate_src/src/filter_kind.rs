#![allow(
    clippy::single_call_fn,
    reason = "the filter projection is a physical boundary between descriptors and emitters"
)]
pub(super) fn filter_kind<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>(
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
