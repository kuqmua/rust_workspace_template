#![allow(
    clippy::single_call_fn,
    reason = "the Serde projection is a physical boundary between descriptors and emitters"
)]
pub(super) fn wire_kind<CanBeNl, CanBePk, FltKind, PgName, WireKind>(
    spec: crate::model::PgTypeSpec<CanBeNl, CanBePk, FltKind, PgName, WireKind>,
) -> WireKind {
    spec.wire_kind
}
