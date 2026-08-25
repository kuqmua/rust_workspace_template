use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

mod request;
mod reset;

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(in crate::domain_types::app) fn AdminSettingsView(
    admin: server_admin_contract::domain_types::AuthenticatedAdmin,
    page: server_admin_contract::domain_types::AdminSettingsView,
) -> impl leptos::prelude::IntoView {
    let can_update = bool::from(admin.has_permission(
        server_admin_contract::domain_types::AdminPermission::SystemSettingsUpdate,
    ));
    let values =
        crate::domain_types::shared::settings::values::AdminSettingsFormValues::from(&page);
    let signals =
        crate::domain_types::shared::settings::signals::AdminSettingsFormSignals::new(&values);
    leptos::view! {
        <section class="settings-grid" data-renderer="csr"><crate::domain_types::ui::card::AdminCard variant=crate::domain_types::ui::card::AdminCardVariant::Settings><form class="settings-form" on:submit=move |event| {
            event.prevent_default();
            request::save(signals);
        }>
            {crate::domain_types::shared::settings::input::admin_setting_inputs(signals, crate::domain_types::shared::settings::input::AdminSettingDisabled::from(!can_update))}
            <crate::domain_types::ui::card::AdminCardFooter>
                <crate::domain_types::ui::button::AdminButton disabled=!can_update>"Save settings"</crate::domain_types::ui::button::AdminButton>
                <crate::domain_types::ui::alert_dialog::AdminAlertDialog id=String::from("reset-settings-dialog") title="Reset settings?" description="All administrator settings will return to the template defaults." trigger="Reset to template defaults" confirm="Reset settings" disabled=!can_update on_confirm=leptos::prelude::Callback::new(move |()| {
                    reset::reset();
                }) />
            </crate::domain_types::ui::card::AdminCardFooter>
        </form></crate::domain_types::ui::card::AdminCard></section>
    }
}
