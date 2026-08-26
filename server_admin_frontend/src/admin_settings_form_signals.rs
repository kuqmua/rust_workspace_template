#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::single_call_fn,
    reason = "the settings signal collection keeps construction before indexed access and centralizes form state for CSR and SSR consumers"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct AdminSettingsFormSignals(
    [crate::domain_types::with_owner::input::LeptosAdminInputSignal;
        server_admin_contract::domain_types::AdminSetting::COUNT],
);
impl AdminSettingsFormSignals {
    pub(crate) fn new(
        values: &super::values::admin_settings_form_values::AdminSettingsFormValues,
    ) -> Self {
        Self::from(
            server_admin_contract::domain_types::AdminSetting::ALL.map(|setting| {
                crate::domain_types::with_owner::input::LeptosAdminInputSignal::from(
                    leptos::prelude::RwSignal::new(values.get(setting).as_ref().to_owned()),
                )
            }),
        )
    }
    pub(crate) const fn get(
        self,
        setting: server_admin_contract::domain_types::AdminSetting,
    ) -> crate::domain_types::with_owner::input::LeptosAdminInputSignal {
        #[allow(
            clippy::indexing_slicing,
            reason = "UnitEnumIndex generates a total index below AdminSetting::COUNT"
        )]
        self.0[setting.index()]
    }
}
