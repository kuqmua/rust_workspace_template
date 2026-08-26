#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "sibling emitters read this private operation descriptor directly"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(in super::super) struct OperationDsc<
    Capability,
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
> {
    pub(in super::super) http_method: HttpMethod,
    pub(in super::super) idempotency_capable: Capability,
    pub(in super::super) operation: Operation,
    pub(in super::super) operation_kind: OperationKind,
    pub(in super::super) optimistic_concurrency_capable: Capability,
    pub(in super::super) permission_action: PermissionAction,
    pub(in super::super) success_status_code: StatusCode,
}
#[cfg(test)]
mod tests {
    #[test]
    fn operation_descriptor_keeps_transport_permission_and_capabilities_together() {
        let spec = super::OperationDsc {
            http_method: constants_str::PATCH,
            idempotency_capable: true,
            operation: constants_str::UO,
            operation_kind: constants_str::UPDATE_ONE,
            optimistic_concurrency_capable: true,
            permission_action: constants_str::PG_CRUD_UPDATE_PERMISSION_ACTION,
            success_status_code: 200u16,
        };
        assert_eq!(spec.http_method, "PATCH");
        assert!(spec.idempotency_capable);
        assert_eq!(spec.operation, "uo");
        assert_eq!(spec.operation_kind, "update_one");
        assert!(spec.optimistic_concurrency_capable);
        assert_eq!(spec.permission_action, "update");
        assert_eq!(spec.success_status_code, 200u16);
    }
}
