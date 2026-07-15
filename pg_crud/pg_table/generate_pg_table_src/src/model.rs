#![allow(clippy::field_scoped_visibility_modifiers)] // sibling emitters read the private descriptor directly while it remains hidden outside the generator
#[derive(Clone, Copy)]
pub(super) struct OperationDsc<
    Capability,
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
> {
    pub(super) http_method: HttpMethod,
    pub(super) idempotency_capable: Capability,
    pub(super) operation: Operation,
    pub(super) operation_kind: OperationKind,
    pub(super) optimistic_concurrency_capable: Capability,
    pub(super) permission_action: PermissionAction,
    pub(super) success_status_code: StatusCode,
}
#[cfg(test)]
mod tests {
    #[test]
    fn operation_descriptor_keeps_transport_permission_and_capabilities_together() {
        let spec = super::OperationDsc {
            http_method: str_constants::PATCH,
            idempotency_capable: true,
            operation: str_constants::UO,
            operation_kind: str_constants::UPDATE_ONE,
            optimistic_concurrency_capable: true,
            permission_action: str_constants::PG_CRUD_UPDATE_PERMISSION_ACTION,
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
