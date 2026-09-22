pub(super) const fn route_success_status<
    ErrorVariants,
    HttpMethod,
    Logic,
    Operation,
    RuleAction,
    StatusCode,
>(
    operation_descriptor: &crate::operation_descriptor::OperationDescriptor<
        ErrorVariants,
        HttpMethod,
        Logic,
        Operation,
        RuleAction,
        StatusCode,
    >,
) -> StatusCode
where
    StatusCode: Copy,
{
    *operation_descriptor.get_success_status_code()
}
