#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_wire_enum::WireEnum,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(try_from = "String")]
#[wire_enum(
    ref_type = crate::admin_data_table_str_ref::AdminDataTableStrRef,
    error_message = constants_str::UNKNOWN_ADMINISTRATOR_DATA_TABLE,
)]
pub enum AdminPrefixedDataTable {
    #[wire("users")]
    Users,
    #[wire("roles")]
    Roles,
    #[wire("permissions")]
    Permissions,
    #[wire("audit_log")]
    AuditLog,
    #[wire("system_settings")]
    SystemSettings,
}
impl From<AdminPrefixedDataTable> for crate::admin_data_table::AdminDataTable {
    fn from(value: AdminPrefixedDataTable) -> Self {
        match value {
            AdminPrefixedDataTable::Users => Self::Users,
            AdminPrefixedDataTable::Roles => Self::Roles,
            AdminPrefixedDataTable::Permissions => Self::Permissions,
            AdminPrefixedDataTable::AuditLog => Self::AuditLog,
            AdminPrefixedDataTable::SystemSettings => Self::SystemSettings,
        }
    }
}

impl std::fmt::Display for AdminPrefixedDataTable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str().get())
    }
}

impl TryFrom<String> for AdminPrefixedDataTable {
    type Error = AdminPrefixedDataTableTryFromStrError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
