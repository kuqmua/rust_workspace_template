#![allow(
    clippy::single_call_fn,
    reason = "the catalog projection is a physical boundary between descriptors and emitters"
)]
pub(super) fn pg_name<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>(
    spec: crate::model::PgTypeSpec<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>,
) -> PgName {
    spec.pg_name
}
