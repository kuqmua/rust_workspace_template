use crate::domain_types::generated_tables::StdOptionalOptionalAdminSystemSettingsWhereMany;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub(crate) struct DataSystemSettingsFlt(pub(super) StdOptionalOptionalAdminSystemSettingsWhereMany);
