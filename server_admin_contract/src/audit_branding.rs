#[path = "admin_audit_details_bytes.rs"]
mod admin_audit_details_bytes;
#[path = "admin_audit_details_max_bytes.rs"]
mod admin_audit_details_max_bytes;
#[path = "admin_audit_details_too_large.rs"]
mod admin_audit_details_too_large;
#[path = "admin_audit_timestamp.rs"]
mod admin_audit_timestamp;
#[path = "admin_default_route.rs"]
mod admin_default_route;
#[path = "admin_main_logo.rs"]
mod admin_main_logo;
#[path = "admin_organization_contacts.rs"]
mod admin_organization_contacts;
#[path = "admin_organization_name.rs"]
mod admin_organization_name;
#[path = "admin_primary_color.rs"]
mod admin_primary_color;
#[path = "admin_site_name.rs"]
mod admin_site_name;
#[path = "admin_support_url.rs"]
mod admin_support_url;
#[path = "admin_tab_title.rs"]
mod admin_tab_title;
#[path = "serde_json_admin_audit_details.rs"]
mod serde_json_admin_audit_details;

pub use admin_audit_details_bytes::AdminAuditDetailsBytes;
pub use admin_audit_details_max_bytes::ADMIN_AUDIT_DETAILS_MAX_BYTES;
pub use admin_audit_details_too_large::AdminAuditDetailsTooLarge;
pub use admin_audit_timestamp::AdminAuditTimestamp;
pub use admin_default_route::AdminDefaultRoute;
pub use admin_main_logo::AdminMainLogo;
pub use admin_organization_contacts::AdminOrganizationContacts;
pub use admin_organization_name::AdminOrganizationName;
pub use admin_primary_color::AdminPrimaryColor;
pub use admin_site_name::AdminSiteName;
pub use admin_support_url::AdminSupportUrl;
pub use admin_tab_title::AdminTabTitle;
pub use serde_json_admin_audit_details::SerdeJsonAdminAuditDetails;

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
