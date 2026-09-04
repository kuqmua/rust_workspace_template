pub(crate) const fn optimistic_concurrency_capable<
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
) -> crate::optimistic_concurrency_capability::OptimisticConcurrencyCapability {
    *operation_descriptor.get_optimistic_concurrency_capable()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_returns_optimistic_concurrency_capability() {
        let descriptor = crate::operation_descriptor::OperationDescriptor::new(
            (),
            (),
            crate::idempotency_capability::IdempotencyCapability::from(false),
            (),
            (),
            crate::optimistic_concurrency_capability::OptimisticConcurrencyCapability::from(true),
            (),
            (),
        );
        assert!(bool::from(
            crate::optimistic_concurrency_capable::optimistic_concurrency_capable(&descriptor)
        ));
    }
}
