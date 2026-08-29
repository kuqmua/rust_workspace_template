#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "sibling emitters read this private operation descriptor directly"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) struct OperationDsc<
    Capability,
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
> {
    pub(crate) http_method: HttpMethod,
    pub(crate) idempotency_capable: Capability,
    pub(crate) operation: Operation,
    pub(crate) operation_kind: OperationKind,
    pub(crate) optimistic_concurrency_capable: Capability,
    pub(crate) permission_action: PermissionAction,
    pub(crate) success_status_code: StatusCode,
}
#[cfg(test)]
mod tests {
    #[test]
    fn operation_descriptor_keeps_transport_permission_and_capabilities_together() {
        let spec = crate::operation_dsc::OperationDsc {
            http_method: constants_str::catalog::PATCH,
            idempotency_capable: true,
            operation: constants_str::catalog::UO,
            operation_kind: constants_str::catalog::UPDATE_ONE,
            optimistic_concurrency_capable: true,
            permission_action: constants_str::catalog::PG_CRUD_UPDATE_PERMISSION_ACTION,
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
