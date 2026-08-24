#![allow(
    clippy::single_call_fn,
    reason = "the Serde projection is a physical boundary between descriptors and emitters"
)]
pub(super) fn wire_kind<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>(
    spec: crate::model::PgTypeSpec<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>,
) -> WireKind {
    spec.wire_kind
}
