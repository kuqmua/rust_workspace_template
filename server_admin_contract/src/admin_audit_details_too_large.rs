#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum AdminAuditDetailsTooLarge {
    #[error(
        "administrator audit details contain {} bytes, maximum is {} bytes",
        .0.get(),
        crate::admin_audit_details_max_bytes::ADMIN_AUDIT_DETAILS_MAX_BYTES
    )]
    TooLarge(crate::admin_audit_details_bytes::AdminAuditDetailsBytes),
}

impl From<crate::admin_audit_details_bytes::AdminAuditDetailsBytes> for AdminAuditDetailsTooLarge {
    fn from(value: crate::admin_audit_details_bytes::AdminAuditDetailsBytes) -> Self {
        Self::TooLarge(value)
    }
}

impl AdminAuditDetailsTooLarge {
    #[must_use]
    pub const fn actual_bytes(self) -> crate::admin_audit_details_bytes::AdminAuditDetailsBytes {
        match self {
            Self::TooLarge(value) => value,
        }
    }
    #[must_use]
    pub fn maximum_bytes(self) -> crate::admin_audit_details_bytes::AdminAuditDetailsBytes {
        crate::admin_audit_details_bytes::AdminAuditDetailsBytes::from(
            crate::admin_audit_details_max_bytes::ADMIN_AUDIT_DETAILS_MAX_BYTES,
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_audit_size_error_accessors_preserve_actual_bytes_and_shared_maximum() {
        assert!(
            [
                0usize,
                crate::admin_audit_details_max_bytes::ADMIN_AUDIT_DETAILS_MAX_BYTES,
                crate::admin_audit_details_max_bytes::ADMIN_AUDIT_DETAILS_MAX_BYTES
                    .saturating_add(2usize)
            ]
            .into_iter()
            .all(|actual| {
                let admin_audit_details_bytes =
                    crate::admin_audit_details_bytes::AdminAuditDetailsBytes::from(actual);
                let error = super::AdminAuditDetailsTooLarge::from(admin_audit_details_bytes);
                error.actual_bytes() == admin_audit_details_bytes
                    && error.maximum_bytes().get()
                        == crate::admin_audit_details_max_bytes::ADMIN_AUDIT_DETAILS_MAX_BYTES
            })
        );
    }
}
