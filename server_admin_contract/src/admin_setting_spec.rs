#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdminSettingSpec {
    pub(super) label: crate::admin_setting_label::AdminSettingLabel,
    pub(super) name: crate::admin_setting_name::AdminSettingName,
    pub(super) input_kind: crate::admin_setting_input_kind::AdminSettingInputKind,
    pub(super) optionality: crate::admin_setting_optionality::AdminSettingOptionality,
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
