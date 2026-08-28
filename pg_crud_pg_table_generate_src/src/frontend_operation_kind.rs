pub(super) const fn frontend_operation_kind<
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
    crate::domain_types::route_operation_kind::route_operation_kind(dsc)
}
