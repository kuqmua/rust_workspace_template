#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    generate_constructor::New,
)]
pub struct AdminSettingSpec {
    label: crate::admin_setting_label::AdminSettingLabel,
    name: crate::admin_setting_name::AdminSettingName,
    input_kind: crate::admin_setting_input_kind::AdminSettingInputKind,
    optionality: crate::admin_setting_optionality::AdminSettingOptionality,
}

impl AdminSettingSpec {
    #[must_use]
    pub const fn input_kind(self) -> crate::admin_setting_input_kind::AdminSettingInputKind {
        self.input_kind
    }
    #[must_use]
    pub const fn label(self) -> crate::admin_setting_label::AdminSettingLabel {
        self.label
    }
    #[must_use]
    pub const fn name(self) -> crate::admin_setting_name::AdminSettingName {
        self.name
    }
    #[must_use]
    pub const fn optionality(self) -> crate::admin_setting_optionality::AdminSettingOptionality {
        self.optionality
    }
    #[must_use]
    pub fn required(self) -> crate::admin_bool::AdminBool {
        crate::admin_bool::AdminBool::from(matches!(
            self.optionality,
            crate::admin_setting_optionality::AdminSettingOptionality::Required
        ))
    }
}
