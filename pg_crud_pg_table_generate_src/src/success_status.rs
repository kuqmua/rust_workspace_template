pub(super) const fn success_status<
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
>(
    operation_descriptor: &crate::operation_descriptor::OperationDescriptor<
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
    crate::route_success_status::route_success_status(operation_descriptor)
}
