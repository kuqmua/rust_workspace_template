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
    super::ADMIN_AUDIT_DETAILS_MAX_BYTES
)]
pub struct AdminAuditDetailsTooLarge(super::AdminAuditDetailsBytes);

impl AdminAuditDetailsTooLarge {
    #[must_use]
    pub const fn actual_bytes(self) -> super::AdminAuditDetailsBytes {
        self.0
    }
    #[must_use]
    pub fn maximum_bytes(self) -> super::AdminAuditDetailsBytes {
        super::AdminAuditDetailsBytes::from(super::ADMIN_AUDIT_DETAILS_MAX_BYTES)
    }
}
