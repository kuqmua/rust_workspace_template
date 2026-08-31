#[derive(
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    generate_accessor::Getters,
)]
pub(crate) struct AdminAuditSuccessRef<'value_lt> {
    action: crate::admin_audit_action::AdminAuditAction,
    login: &'value_lt server_admin_contract::admin_login::AdminLogin,
    resource: crate::admin_audit_resource::AdminAuditResource,
    resource_id: crate::admin_audit_resource_id::AdminAuditResourceId,
    user_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
}
