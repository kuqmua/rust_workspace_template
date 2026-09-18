#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    proc_macro_newtype_from_inner::FromInner,
    utoipa::ToSchema,
)]
#[serde(from = "[crate::admin_read_audit_log_column::AdminReadAuditLogColumn; 9]")]
pub struct AdminReadAuditLogSelection(
    [crate::admin_read_audit_log_column::AdminReadAuditLogColumn; 9],
);

impl Default for AdminReadAuditLogSelection {
    fn default() -> Self {
        Self::from([
            crate::admin_read_audit_log_column::AdminReadAuditLogColumn::Id(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_audit_log_column::AdminReadAuditLogColumn::UserId(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_audit_log_column::AdminReadAuditLogColumn::UserLogin(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_audit_log_column::AdminReadAuditLogColumn::Action(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_audit_log_column::AdminReadAuditLogColumn::Resource(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_audit_log_column::AdminReadAuditLogColumn::ResourceId(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_audit_log_column::AdminReadAuditLogColumn::RequestId(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_audit_log_column::AdminReadAuditLogColumn::Succeeded(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_audit_log_column::AdminReadAuditLogColumn::CreatedAt(
                crate::admin_no_body::AdminNoBody,
            ),
        ])
    }
}
