use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

mod request;
mod reset;

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(in crate::app) fn AdminSettingsView(
    admin: server_admin_contract::AuthenticatedAdmin,
    page: server_admin_contract::AdminSettingsView,
) -> impl leptos::prelude::IntoView {
    let can_update = bool::from(
        admin.has_permission(server_admin_contract::AdminPermission::SystemSettingsUpdate),
    );
    let values = crate::shared::settings::values::AdminSettingsFormValues::from(&page);
    let signals = crate::shared::settings::signals::AdminSettingsFormSignals::new(&values);
    leptos::view! {
        <section class="settings-grid" data-renderer="csr"><article class="settings-card"><form class="settings-form" on:submit=move |event| {
            event.prevent_default();
            request::save(signals);
        }>
            {crate::shared::settings::input::admin_setting_inputs(signals, crate::shared::settings::input::AdminSettingDisabled::from(!can_update))}
            <button type="submit" disabled=!can_update>"Save settings"</button>
            <button type="button" disabled=!can_update on:click=move |_event| {
                reset::reset();
            }>"Reset to template defaults"</button>
        </form></article></section>
    }
}
