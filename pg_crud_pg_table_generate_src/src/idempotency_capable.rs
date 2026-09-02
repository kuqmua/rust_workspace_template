pub(crate) const fn idempotency_capable<
    Capability,
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
>(
    operation_descriptor: &crate::operation_descriptor::OperationDescriptor<
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
    *operation_descriptor.get_idempotency_capable()
}
#[cfg(test)]
mod tests {
    fn descriptor(
        idempotency_capable: bool,
        optimistic_concurrency_capable: bool,
    ) -> crate::operation_descriptor::OperationDescriptor<bool, (), (), (), (), ()> {
        crate::operation_descriptor::OperationDescriptor::new(
            (),
            idempotency_capable,
            (),
            (),
            optimistic_concurrency_capable,
            (),
            (),
        )
    }

    #[test]
    fn test_capability_projection_returns_each_independent_descriptor_flag() {
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
