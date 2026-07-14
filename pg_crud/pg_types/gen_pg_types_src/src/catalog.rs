#![allow(
    clippy::single_call_fn,
    reason = "the catalog projection is a physical boundary between descriptors and emitters"
)]
pub(super) fn pg_name<CanBeNl, CanBePk, FltKind, PgName, WireKind>(
    spec: crate::model::PgTypeSpec<CanBeNl, CanBePk, FltKind, PgName, WireKind>,
) -> PgName {
    spec.pg_name
}
