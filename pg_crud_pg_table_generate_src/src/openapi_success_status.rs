#![allow(
    clippy::single_call_fn,
    reason = "OpenAPI success-status emission has one source assembly owner"
)]
pub(in crate::domain_types) const fn openapi_success_status<
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
) -> StatusCode
where
    StatusCode: Copy,
{
    crate::domain_types::route_success_status::route_success_status(dsc)
}
