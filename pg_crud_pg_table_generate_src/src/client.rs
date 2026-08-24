#![allow(
    clippy::single_call_fn,
    reason = "client transport emission has a private physical boundary from route metadata"
)]
pub(super) const fn http_method<
    Capability,
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
>(
    dsc: &crate::model::OperationDsc<
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
    crate::route::http_method(dsc)
}
