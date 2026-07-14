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
