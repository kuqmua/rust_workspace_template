pub(super) fn wire_kind<CanBeNl, CanBePk, FltKind, PgName, WireKind>(
    spec: crate::model::PgTypeSpec<CanBeNl, CanBePk, FltKind, PgName, WireKind>,
) -> WireKind {
    spec.wire_kind
}
