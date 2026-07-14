#![allow(
    clippy::single_call_fn,
    reason = "the filter projection is a physical boundary between descriptors and emitters"
)]
pub(super) fn flt_kind<CanBeNl, CanBePk, FltKind, PgName, WireKind>(
    spec: crate::model::PgTypeSpec<CanBeNl, CanBePk, FltKind, PgName, WireKind>,
) -> FltKind {
    spec.flt_kind
}
