#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminSettingOptionality {
    Clearable(crate::admin_optional_setting::AdminOptionalSetting),
    Required,
}
