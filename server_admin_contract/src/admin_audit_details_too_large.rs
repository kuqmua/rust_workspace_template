#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
    newtype::FromInner,
)]
#[error(
    "administrator audit details contain {} bytes, maximum is {} bytes",
    .0.0,
    crate::admin_audit_details_max_bytes::ADMIN_AUDIT_DETAILS_MAX_BYTES
)]
pub struct AdminAuditDetailsTooLarge(crate::admin_audit_details_bytes::AdminAuditDetailsBytes);

impl AdminAuditDetailsTooLarge {
    #[must_use]
    pub const fn actual_bytes(self) -> crate::admin_audit_details_bytes::AdminAuditDetailsBytes {
        self.0
    }
    #[must_use]
    pub fn maximum_bytes(self) -> crate::admin_audit_details_bytes::AdminAuditDetailsBytes {
        crate::admin_audit_details_bytes::AdminAuditDetailsBytes::from(
            crate::admin_audit_details_max_bytes::ADMIN_AUDIT_DETAILS_MAX_BYTES,
        )
    }
}
