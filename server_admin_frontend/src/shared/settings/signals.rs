#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::single_call_fn,
    reason = "the settings signal collection keeps construction before indexed access and centralizes form state for CSR and SSR consumers"
)]

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct AdminSettingsFormSignals(
    [super::input::LeptosAdminSettingSignal; server_admin_contract::AdminSetting::COUNT],
);
impl AdminSettingsFormSignals {
    pub(crate) fn new(values: &super::values::AdminSettingsFormValues) -> Self {
        Self::from(server_admin_contract::AdminSetting::ALL.map(|setting| {
            super::input::LeptosAdminSettingSignal::from(leptos::prelude::RwSignal::new(
                values.get(setting).as_ref().to_owned(),
            ))
        }))
    }
    pub(crate) const fn get(
        self,
        setting: server_admin_contract::AdminSetting,
    ) -> super::input::LeptosAdminSettingSignal {
        #[allow(
            clippy::indexing_slicing,
            reason = "UnitEnumIndex generates a total index below AdminSetting::COUNT"
        )]
        self.0[setting.index()]
    }
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn optional_settings_to_clear(
        self,
    ) -> Result<
        server_admin_contract::AdminOptionalSettings,
        server_admin_contract::AdminCollectionError,
    > {
        let values = server_admin_contract::AdminSetting::ALL
            .into_iter()
            .filter_map(|setting| match setting.spec().optionality() {
                server_admin_contract::AdminSettingOptionality::Clearable(optional)
                    if self.get(setting).value().as_ref().is_empty() =>
                {
                    Some(optional)
                }
                server_admin_contract::AdminSettingOptionality::Clearable(_)
                | server_admin_contract::AdminSettingOptionality::Required => None,
            })
            .collect::<Vec<_>>();
        server_admin_contract::AdminOptionalSettings::try_from(values)
    }
}
