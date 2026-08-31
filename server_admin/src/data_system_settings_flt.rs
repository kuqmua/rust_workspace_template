#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    newtype::FromInner,
    newtype::IntoInner,
    generate_accessor::Getters,
)]
pub(crate) struct DataSystemSettingsFlt(
    crate::admin_system_settings::StdOptionalOptionalAdminSystemSettingsWhereMany,
);
