#![allow(
    clippy::single_call_fn,
    reason = "route projections are private physical boundaries shared by transport emitters"
)]
pub(super) const fn http_method<
    Capability,
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
>(
    dsc: &crate::domain_types::table::OperationDsc<
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
    dsc.http_method
}
