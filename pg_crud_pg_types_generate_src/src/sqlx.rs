#![allow(
    clippy::single_call_fn,
    reason = "SQLx capability projections are a physical boundary between descriptors and emitters"
)]
pub(super) fn can_be_nullable<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>(
    spec: crate::model::PgTypeSpec<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>,
) -> CanBeNullable {
    spec.can_be_nullable
}
pub(super) fn can_be_primary_key<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>(
    spec: crate::model::PgTypeSpec<CanBeNullable, CanBePrimaryKey, FilterKind, PgName, WireKind>,
) -> CanBePrimaryKey {
    spec.can_be_primary_key
}
