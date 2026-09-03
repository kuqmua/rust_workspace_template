#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the settings signal collection keeps construction before indexed access and centralizes form state for CSR and SSR consumers"
)]

#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub(crate) struct AdminSettingsFormSignals(
    [crate::leptos_admin_input_signal::LeptosAdminInputSignal;
        server_admin_contract::admin_setting::AdminSetting::COUNT],
);
impl AdminSettingsFormSignals {
    #[allow(
        clippy::single_call_fn,
        reason = "admin settings form signals remains a named owner because its boundary role is clearer and directly testable"
    )]
    pub(crate) fn new(
        admin_settings_form_values: &crate::admin_settings_form_values::AdminSettingsFormValues,
    ) -> Self {
        Self::from(
            server_admin_contract::admin_setting::AdminSetting::ALL.map(|setting| {
                crate::leptos_admin_input_signal::LeptosAdminInputSignal::from(
                    leptos::prelude::RwSignal::new(
                        admin_settings_form_values.get(setting).as_ref().to_owned(),
                    ),
                )
            }),
        )
    }
    pub(crate) const fn get(
        self,
        admin_setting: server_admin_contract::admin_setting::AdminSetting,
    ) -> crate::leptos_admin_input_signal::LeptosAdminInputSignal {
        #[allow(
            clippy::indexing_slicing,
            reason = "UnitEnumIndex generates a total index below AdminSetting::COUNT"
        )]
        self.0[admin_setting.index()]
    }
}
