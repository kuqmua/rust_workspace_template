#![allow(
    clippy::single_call_fn,
    reason = "frontend contract emission has a private physical boundary from route metadata"
)]
pub(super) const fn http_method<Capability, HttpMethod, Op, OpKind, PermissionAction, StatusCode>(
    dsc: &crate::model::OpDsc<Capability, HttpMethod, Op, OpKind, PermissionAction, StatusCode>,
) -> HttpMethod
where
    HttpMethod: Copy,
{
    crate::route::http_method(dsc)
}
pub(super) const fn operation_kind<
    Capability,
    HttpMethod,
    Op,
    OpKind,
    PermissionAction,
    StatusCode,
>(
    dsc: &crate::model::OpDsc<Capability, HttpMethod, Op, OpKind, PermissionAction, StatusCode>,
) -> OpKind
where
    OpKind: Copy,
{
    crate::route::operation_kind(dsc)
}
pub(super) const fn permission_action<
    Capability,
    HttpMethod,
    Op,
    OpKind,
    PermissionAction,
    StatusCode,
>(
    dsc: &crate::model::OpDsc<Capability, HttpMethod, Op, OpKind, PermissionAction, StatusCode>,
) -> PermissionAction
where
    PermissionAction: Copy,
{
    crate::route::permission_action(dsc)
}
pub(super) const fn success_status<
    Capability,
    HttpMethod,
    Op,
    OpKind,
    PermissionAction,
    StatusCode,
>(
    dsc: &crate::model::OpDsc<Capability, HttpMethod, Op, OpKind, PermissionAction, StatusCode>,
) -> StatusCode
where
    StatusCode: Copy,
{
    crate::route::success_status(dsc)
}
