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

#[cfg(test)]
mod tests {
    #[test]
    fn test_setting_spec_required_flag_preserves_clearable_and_input_metadata() {
        let label = crate::admin_setting_label::AdminSettingLabel::from(constants_str::ADMIN);
        let name = crate::admin_setting_name::AdminSettingName::from(constants_str::LOGIN);
        assert!([
            crate::admin_setting_input_kind::AdminSettingInputKind::Text,
            crate::admin_setting_input_kind::AdminSettingInputKind::TextArea,
            crate::admin_setting_input_kind::AdminSettingInputKind::Url,
        ].into_iter().all(|input_kind| {
            let required = super::AdminSettingSpec::new(label, name, input_kind, crate::admin_setting_optionality::AdminSettingOptionality::Required);
            required.required() == crate::admin_bool::AdminBool::from(true)
                && required.input_kind() == input_kind
                && crate::admin_optional_setting::AdminOptionalSetting::ALL.iter().all(|optional_setting| {
                    let optionality = crate::admin_setting_optionality::AdminSettingOptionality::Clearable(*optional_setting);
                    let clearable = super::AdminSettingSpec::new(label, name, input_kind, optionality);
                    clearable.required() == crate::admin_bool::AdminBool::from(false)
                        && clearable.optionality() == optionality
                        && clearable.label() == label && clearable.name() == name
                        && clearable.input_kind() == input_kind
                })
        }));
    }
}
