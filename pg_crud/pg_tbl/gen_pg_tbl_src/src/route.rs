#![allow(
    clippy::single_call_fn,
    reason = "route projections are private physical boundaries shared by transport emitters"
)]
pub(super) const fn http_method<Capability, HttpMethod, Op, OpKind, PermissionAction, StatusCode>(
    dsc: &crate::model::OpDsc<Capability, HttpMethod, Op, OpKind, PermissionAction, StatusCode>,
) -> HttpMethod
where
    HttpMethod: Copy,
{
    dsc.http_method
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
    dsc.operation_kind
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
    dsc.permission_action
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
    dsc.success_status_code
}
