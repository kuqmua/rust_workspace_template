pub(super) const fn success_status<
    ErrorVariants,
    HttpMethod,
    Logic,
    Operation,
    PermissionAction,
    StatusCode,
>(
    operation_descriptor: &crate::operation_descriptor::OperationDescriptor<
        ErrorVariants,
        HttpMethod,
        Logic,
        Operation,
        PermissionAction,
        StatusCode,
    >,
) -> StatusCode
where
    StatusCode: Copy,
{
    crate::route_success_status::route_success_status(operation_descriptor)
}
