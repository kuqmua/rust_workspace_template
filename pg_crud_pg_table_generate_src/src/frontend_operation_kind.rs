#![allow(clippy::single_call_fn)] // frontend projection is a private physical emitter boundary

pub(super) const fn operation_kind<
    Capability,
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
>(
    dsc: &crate::domain_types::table::operation_dsc::OperationDsc<
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
    crate::domain_types::route_operation_kind::operation_kind(dsc)
}
