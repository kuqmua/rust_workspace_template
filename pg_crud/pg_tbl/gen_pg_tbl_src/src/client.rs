#![allow(
    clippy::single_call_fn,
    reason = "client transport emission has a private physical boundary from route metadata"
)]
pub(super) const fn http_method<Capability, HttpMethod, Op, OpKind, PermissionAction, StatusCode>(
    dsc: &crate::model::OpDsc<Capability, HttpMethod, Op, OpKind, PermissionAction, StatusCode>,
) -> HttpMethod
where
    HttpMethod: Copy,
{
    crate::route::http_method(dsc)
}
