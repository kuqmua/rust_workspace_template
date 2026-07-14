#![allow(clippy::field_scoped_visibility_modifiers)] // sibling emitters read the private descriptor directly while it remains hidden outside the generator
#[derive(Clone, Copy)]
pub(super) struct OpDsc<Capability, HttpMethod, Op, OpKind, PermissionAction, StatusCode> {
    pub(super) http_method: HttpMethod,
    pub(super) idempotency_capable: Capability,
    pub(super) op: Op,
    pub(super) operation_kind: OpKind,
    pub(super) optimistic_concurrency_capable: Capability,
    pub(super) permission_action: PermissionAction,
    pub(super) success_status_code: StatusCode,
}
#[cfg(test)]
mod tests {
    #[test]
    fn operation_descriptor_keeps_transport_permission_and_capabilities_together() {
        let spec = super::OpDsc {
            http_method: "PATCH",
            idempotency_capable: true,
            op: "uo",
            operation_kind: "update_one",
            optimistic_concurrency_capable: true,
            permission_action: "update",
            success_status_code: 200u16,
        };
        assert_eq!(spec.http_method, "PATCH");
        assert!(spec.idempotency_capable);
        assert_eq!(spec.op, "uo");
        assert_eq!(spec.operation_kind, "update_one");
        assert!(spec.optimistic_concurrency_capable);
        assert_eq!(spec.permission_action, "update");
        assert_eq!(spec.success_status_code, 200u16);
    }
}
