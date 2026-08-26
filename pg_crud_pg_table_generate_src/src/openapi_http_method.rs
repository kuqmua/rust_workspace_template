#![allow(
    clippy::single_call_fn,
    reason = "OpenAPI emission has a private physical boundary from route metadata"
)]
pub(in crate::domain_types) const fn http_method<
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
) -> HttpMethod
where
    HttpMethod: Copy,
{
    crate::domain_types::route_http_method::route_http_method(dsc)
}
