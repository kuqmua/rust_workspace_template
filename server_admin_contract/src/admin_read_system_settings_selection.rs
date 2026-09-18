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
#[serde(from = "[crate::admin_read_system_settings_column::AdminReadSystemSettingsColumn; 10]")]
pub struct AdminReadSystemSettingsSelection(
    [crate::admin_read_system_settings_column::AdminReadSystemSettingsColumn; 10],
);

impl Default for AdminReadSystemSettingsSelection {
    fn default() -> Self {
        Self::from([
            crate::admin_read_system_settings_column::AdminReadSystemSettingsColumn::Id(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_system_settings_column::AdminReadSystemSettingsColumn::SiteName(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_system_settings_column::AdminReadSystemSettingsColumn::TabTitle(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_system_settings_column::AdminReadSystemSettingsColumn::MainLogo(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_system_settings_column::AdminReadSystemSettingsColumn::PrimaryColor(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_system_settings_column::AdminReadSystemSettingsColumn::DefaultAdminRoute(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_system_settings_column::AdminReadSystemSettingsColumn::OrganizationName(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_system_settings_column::AdminReadSystemSettingsColumn::OrganizationContacts(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_system_settings_column::AdminReadSystemSettingsColumn::SupportUrl(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_system_settings_column::AdminReadSystemSettingsColumn::UpdatedAt(
                crate::admin_no_body::AdminNoBody,
            ),
        ])
    }
}
