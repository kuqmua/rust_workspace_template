use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(crate) fn AdminSettingsView(
    authenticated_admin: server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_settings_view: server_admin_contract::admin_settings_view::AdminSettingsView,
) -> impl leptos::prelude::IntoView {
    let can_update = bool::from(authenticated_admin.has_permission(
        server_admin_contract::admin_permission::AdminPermission::SystemSettingsUpdate,
    ));
    let values =
        crate::admin_settings_form_values::AdminSettingsFormValues::from(&admin_settings_view);
    let signals = crate::admin_settings_form_signals::AdminSettingsFormSignals::new(&values);
    leptos::view! {
        <section class="settings-grid" data-renderer="csr"><crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Settings><form class="settings-form" on:submit=move |event| {
            event.prevent_default();
            crate::save::save(signals);
        }>
            {crate::admin_setting_inputs::admin_setting_inputs(signals, crate::admin_setting_disabled::AdminSettingDisabled::from(!can_update))}
            <crate::admin_card_footer::AdminCardFooter>
                <crate::admin_button::AdminButton bool=!can_update>"Save settings"</crate::admin_button::AdminButton>
                <crate::admin_alert_dialog::AdminAlertDialog string=String::from("reset-settings-dialog") title="Reset settings?" description="All administrator settings will return to the template defaults." trigger="Reset to template defaults" confirm="Reset settings" bool=!can_update callback=leptos::prelude::Callback::new(move |()| {
                    crate::reset::reset();
                }) />
            </crate::admin_card_footer::AdminCardFooter>
        </form></crate::admin_card::AdminCard></section>
    }
}
