#![allow(
    clippy::single_call_fn,
    reason = "SQLx capability projections are a physical boundary between descriptors and emitters"
)]
pub(in crate::domain_types) fn pg_type_can_be_nullable<
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
) -> CanBeNullable {
    spec.can_be_nullable
}
