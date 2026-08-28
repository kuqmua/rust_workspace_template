pub use crate::admin_audit_details_bytes::AdminAuditDetailsBytes;
pub use crate::admin_audit_details_max_bytes::ADMIN_AUDIT_DETAILS_MAX_BYTES;
pub use crate::admin_audit_details_too_large::AdminAuditDetailsTooLarge;
pub use crate::admin_audit_timestamp::AdminAuditTimestamp;
pub use crate::admin_default_route::AdminDefaultRoute;
pub use crate::admin_main_logo::AdminMainLogo;
pub use crate::admin_organization_contacts::AdminOrganizationContacts;
pub use crate::admin_organization_name::AdminOrganizationName;
pub use crate::admin_primary_color::AdminPrimaryColor;
pub use crate::admin_site_name::AdminSiteName;
pub use crate::admin_support_url::AdminSupportUrl;
pub use crate::admin_tab_title::AdminTabTitle;
pub use crate::serde_json_admin_audit_details::SerdeJsonAdminAuditDetails;

#[cfg(test)]
mod tests {
    #[test]
    fn audit_detail_limit_is_stable() {
        assert_eq!(
            super::AdminAuditDetailsTooLarge::from(super::AdminAuditDetailsBytes::from(
                constants_usize::ONE,
            ))
            .maximum_bytes(),
            super::AdminAuditDetailsBytes::from(super::ADMIN_AUDIT_DETAILS_MAX_BYTES),
        );
    }
}
