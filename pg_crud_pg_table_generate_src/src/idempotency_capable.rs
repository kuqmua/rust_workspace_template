pub(crate) const fn idempotency_capable<
    ErrorVariants,
    HttpMethod,
    Logic,
    Operation,
    PermissionAction,
    StatusCode,
>(
    operation_descriptor: &crate::operation_descriptor::OperationDescriptor<
        ErrorVariants,
        HttpMethod,
        Logic,
        Operation,
        PermissionAction,
        StatusCode,
    >,
) -> crate::idempotency_capability::IdempotencyCapability {
    *operation_descriptor.get_idempotency_capable()
}
#[cfg(test)]
mod tests {
    fn descriptor(
        idempotency_capability: crate::idempotency_capability::IdempotencyCapability,
        optimistic_concurrency_capability: crate::optimistic_concurrency_capability::OptimisticConcurrencyCapability,
    ) -> crate::operation_descriptor::OperationDescriptor<(), (), (), (), (), ()> {
        crate::operation_descriptor::OperationDescriptor::new(
            (),
            (),
            idempotency_capability,
            (),
            (),
            optimistic_concurrency_capability,
            (),
            (),
        )
    }

    #[test]
    fn test_capability_projection_returns_each_independent_descriptor_flag() {
        let idempotent = descriptor(
            crate::idempotency_capability::IdempotencyCapability::from(true),
            crate::optimistic_concurrency_capability::OptimisticConcurrencyCapability::from(false),
        );
        assert!(bool::from(crate::idempotency_capable::idempotency_capable(
            &idempotent
        )));
        assert!(!bool::from(
            crate::optimistic_concurrency_capable::optimistic_concurrency_capable(&idempotent)
        ));

        let optimistic = descriptor(
            crate::idempotency_capability::IdempotencyCapability::from(false),
            crate::optimistic_concurrency_capability::OptimisticConcurrencyCapability::from(true),
        );
        assert!(!bool::from(
            crate::idempotency_capable::idempotency_capable(&optimistic)
        ));
        assert!(bool::from(
            crate::optimistic_concurrency_capable::optimistic_concurrency_capable(&optimistic)
        ));
    }
}
