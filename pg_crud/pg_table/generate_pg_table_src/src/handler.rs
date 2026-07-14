pub(super) const fn success_status<
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
) -> StatusCode
where
    StatusCode: Copy,
{
    crate::route::success_status(dsc)
}
