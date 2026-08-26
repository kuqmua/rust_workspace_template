#![allow(clippy::single_call_fn)] // route projection is a private physical emitter boundary

pub(super) const fn permission_action<
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
) -> PermissionAction
where
    PermissionAction: Copy,
{
    dsc.permission_action
}
