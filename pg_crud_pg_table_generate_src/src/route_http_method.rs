pub(super) const fn route_http_method<
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
) -> HttpMethod
where
    HttpMethod: Copy,
{
    *operation_descriptor.get_http_method()
}
