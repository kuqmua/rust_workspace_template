#![allow(clippy::single_call_fn)] // route projection is a private physical emitter boundary

pub(super) const fn operation_kind<
    Capability,
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
>(
    dsc: &crate::domain_types::table::OperationDsc<
        Capability,
        HttpMethod,
        Operation,
        OperationKind,
        PermissionAction,
        StatusCode,
    >,
) -> OperationKind
where
    OperationKind: Copy,
{
    dsc.operation_kind
}
