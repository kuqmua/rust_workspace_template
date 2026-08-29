pub(crate) const fn idempotency_capable<
    Capability,
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
>(
    dsc: &crate::operation_dsc::OperationDsc<
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
#[cfg(test)]
mod tests {
    fn descriptor(
        idempotency_capable: bool,
        optimistic_concurrency_capable: bool,
    ) -> crate::operation_dsc::OperationDsc<bool, (), (), (), (), ()> {
        crate::operation_dsc::OperationDsc {
            http_method: (),
            idempotency_capable,
            operation: (),
            operation_kind: (),
            optimistic_concurrency_capable,
            permission_action: (),
            success_status_code: (),
        }
    }

    #[test]
    fn capability_projection_returns_each_independent_descriptor_flag() {
        let idempotent = descriptor(true, false);
        assert!(crate::idempotency_capable::idempotency_capable(&idempotent));
        assert!(
            !crate::optimistic_concurrency_capable::optimistic_concurrency_capable(&idempotent)
        );

        let optimistic = descriptor(false, true);
        assert!(!crate::idempotency_capable::idempotency_capable(
            &optimistic
        ));
        assert!(crate::optimistic_concurrency_capable::optimistic_concurrency_capable(&optimistic));
    }
}
