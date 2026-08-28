#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    frontend_contract::UnitEnumCatalog,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminOptionalSetting {
    TabTitle,
    OrganizationName,
    OrganizationContacts,
    SupportUrl,
    PrimaryColor,
    MainLogo,
}
