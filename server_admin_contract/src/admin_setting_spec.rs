#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_new::New,
)]
pub struct AdminSettingSpec {
    #[getters(copy)]
    label: crate::admin_setting_label::AdminSettingLabel,
    #[getters(copy)]
    name: crate::admin_setting_name::AdminSettingName,
    #[getters(copy)]
    input_kind: crate::admin_setting_input_kind::AdminSettingInputKind,
    #[getters(copy)]
    optionality: crate::admin_setting_optionality::AdminSettingOptionality,
}

impl AdminSettingSpec {
    #[must_use]
    pub fn required(self) -> crate::admin_bool::AdminBool {
        crate::admin_bool::AdminBool::from(matches!(
            self.optionality,
            crate::admin_setting_optionality::AdminSettingOptionality::Required
        ))
    }
}
