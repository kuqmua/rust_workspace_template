#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminReadSystemSettingsColumn {
    DefaultAdminRoute(crate::admin_no_body::AdminNoBody),
    Id(crate::admin_no_body::AdminNoBody),
    MainLogo(crate::admin_no_body::AdminNoBody),
    OrganizationContacts(crate::admin_no_body::AdminNoBody),
    OrganizationName(crate::admin_no_body::AdminNoBody),
    PrimaryColor(crate::admin_no_body::AdminNoBody),
    SiteName(crate::admin_no_body::AdminNoBody),
    SupportUrl(crate::admin_no_body::AdminNoBody),
    TabTitle(crate::admin_no_body::AdminNoBody),
    UpdatedAt(crate::admin_no_body::AdminNoBody),
}
