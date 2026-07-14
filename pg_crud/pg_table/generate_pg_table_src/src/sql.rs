pub(super) const fn idempotency_capable<
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
) -> Capability
where
    Capability: Copy,
{
    dsc.idempotency_capable
}
pub(super) const fn optimistic_concurrency_capable<
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
) -> Capability
where
    Capability: Copy,
{
    dsc.optimistic_concurrency_capable
}
