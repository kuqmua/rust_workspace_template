pub(super) const fn route_success_status<
    Capability,
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
>(
    dsc: &crate::operation_dsc::OperationDsc<
        Capability,
        HttpMethod,
        Operation,
        OperationKind,
        PermissionAction,
        StatusCode,
    >,
) -> StatusCode
where
    StatusCode: Copy,
{
    dsc.success_status_code
}
