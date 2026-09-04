#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
)]
pub(crate) struct OperationDescriptor<
    ErrorVariants,
    HttpMethod,
    Logic,
    Operation,
    PermissionAction,
    StatusCode,
> {
    error_variants: ErrorVariants,
    http_method: HttpMethod,
    idempotency_capable: crate::idempotency_capability::IdempotencyCapability,
    logic: Logic,
    operation: Operation,
    optimistic_concurrency_capable:
        crate::optimistic_concurrency_capability::OptimisticConcurrencyCapability,
    permission_action: PermissionAction,
    success_status_code: StatusCode,
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_operation_descriptor_keeps_transport_permission_and_capabilities_together() {
        let spec = crate::operation_descriptor::OperationDescriptor::new(
            constants_str::ERROR,
            constants_str::PATCH,
            crate::idempotency_capability::IdempotencyCapability::from(true),
            constants_str::ERROR,
            constants_str::UO,
            crate::optimistic_concurrency_capability::OptimisticConcurrencyCapability::from(true),
            constants_str::PG_CRUD_UPDATE_PERMISSION_ACTION,
            200u16,
        );
        assert_eq!(*spec.get_http_method(), constants_str::PATCH);
        assert_eq!(*spec.get_error_variants(), constants_str::ERROR);
        assert_eq!(*spec.get_logic(), constants_str::ERROR);
        let idempotency_capability: &crate::idempotency_capability::IdempotencyCapability =
            spec.get_idempotency_capable();
        let optimistic_concurrency_capability: &crate::optimistic_concurrency_capability::OptimisticConcurrencyCapability =
            spec.get_optimistic_concurrency_capable();
        assert!(bool::from(*idempotency_capability));
        assert_eq!(*spec.get_operation(), constants_str::UO);
        assert!(bool::from(*optimistic_concurrency_capability));
        assert_eq!(
            *spec.get_permission_action(),
            constants_str::ADMIN_FIXTURE_AUDIT_ACTION
        );
        assert_eq!(*spec.get_success_status_code(), 200u16);
    }
}
