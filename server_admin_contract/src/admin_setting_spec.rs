#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdminSettingSpec {
    pub(super) label: super::AdminSettingLabel,
    pub(super) name: super::AdminSettingName,
    pub(super) input_kind: super::AdminSettingInputKind,
    pub(super) optionality: super::AdminSettingOptionality,
}

impl AdminSettingSpec {
    #[must_use]
    pub const fn input_kind(self) -> super::AdminSettingInputKind {
        self.input_kind
    }
    #[must_use]
    pub const fn label(self) -> super::AdminSettingLabel {
        self.label
    }
    #[must_use]
    pub const fn name(self) -> super::AdminSettingName {
        self.name
    }
    #[must_use]
    pub const fn optionality(self) -> super::AdminSettingOptionality {
        self.optionality
    }
    #[must_use]
    pub fn required(self) -> super::super::AdminBool {
        super::super::AdminBool::from(matches!(
            self.optionality,
            super::AdminSettingOptionality::Required
        ))
    }
}
