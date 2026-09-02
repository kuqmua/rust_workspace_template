#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
)]
pub(crate) struct OperationDsc<
    Capability,
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
> {
    http_method: HttpMethod,
    idempotency_capable: Capability,
    operation: Operation,
    operation_kind: OperationKind,
    optimistic_concurrency_capable: Capability,
    permission_action: PermissionAction,
    success_status_code: StatusCode,
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_operation_descriptor_keeps_transport_permission_and_capabilities_together() {
        let spec = crate::operation_dsc::OperationDsc::new(
            constants_str::PATCH,
            true,
            constants_str::UO,
            constants_str::UPDATE_ONE,
            true,
            constants_str::PG_CRUD_UPDATE_PERMISSION_ACTION,
            200u16,
        );
        assert_eq!(*spec.get_http_method(), "PATCH");
        assert!(*spec.get_idempotency_capable());
        assert_eq!(*spec.get_operation(), "uo");
        assert_eq!(*spec.get_operation_kind(), "update_one");
        assert!(*spec.get_optimistic_concurrency_capable());
        assert_eq!(*spec.get_permission_action(), "update");
        assert_eq!(*spec.get_success_status_code(), 200u16);
    }
}
