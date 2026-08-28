pub(crate) const fn optimistic_concurrency_capable<
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
) -> Capability
where
    Capability: Copy,
{
    dsc.optimistic_concurrency_capable
}

#[cfg(test)]
mod tests {
    #[test]
    fn returns_optimistic_concurrency_capability() {
        let dsc = crate::domain_types::table::operation_dsc::OperationDsc {
            http_method: (),
            idempotency_capable: false,
            operation: (),
            operation_kind: (),
            optimistic_concurrency_capable: true,
            permission_action: (),
            success_status_code: (),
        };
        assert!(super::optimistic_concurrency_capable(&dsc));
    }
}
