#![allow(
    clippy::single_call_fn,
    reason = "SQLx capability projections are a physical boundary between descriptors and emitters"
)]
pub(super) fn can_be_nl<CanBeNl, CanBePk, FltKind, PgName, WireKind>(
    spec: crate::model::PgTypeSpec<CanBeNl, CanBePk, FltKind, PgName, WireKind>,
) -> CanBeNl {
    spec.can_be_nl
}
pub(super) fn can_be_pk<CanBeNl, CanBePk, FltKind, PgName, WireKind>(
    spec: crate::model::PgTypeSpec<CanBeNl, CanBePk, FltKind, PgName, WireKind>,
) -> CanBePk {
    spec.can_be_pk
}
