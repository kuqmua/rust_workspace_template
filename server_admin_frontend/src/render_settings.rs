#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

#[must_use]
pub(super) fn render_settings(
    view: &server_admin_contract::domain_types::AdminSettingsView,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
    let values = crate::domain_types::shared::settings::values::admin_settings_form_values::AdminSettingsFormValues::from(view);
    let signals =
        crate::domain_types::shared::settings::admin_settings_form_signals::AdminSettingsFormSignals::new(&values);
    let can_update = bool::from(admin.has_permission(
        server_admin_contract::domain_types::AdminPermission::SystemSettingsUpdate,
    ));
    let content_view = leptos::view! {
        <section class="settings-grid"><crate::domain_types::with_owner::card::AdminCard variant=crate::domain_types::with_owner::card::AdminCardVariant::Settings>
        {can_update.then(|| leptos::view! { <form class="settings-form" method="post" action=server_admin_contract::domain_types::AdminHtmlAction::SettingsUpdate.get()>
            {crate::domain_types::shared::settings::input::admin_setting_inputs(signals, crate::domain_types::shared::settings::input::AdminSettingDisabled::from(false))}
            <crate::domain_types::with_owner::card::AdminCardFooter><crate::domain_types::with_owner::button::AdminButton>"Save settings"</crate::domain_types::with_owner::button::AdminButton></crate::domain_types::with_owner::card::AdminCardFooter>
        </form> })}
        {(!can_update).then(|| leptos::view! { <crate::domain_types::with_owner::alert::AdminAlert>"Settings are read-only for this account."</crate::domain_types::with_owner::alert::AdminAlert> })}
        </crate::domain_types::with_owner::card::AdminCard></section>
    };
    let content = super::render_view(content_view);
    super::render_admin_page_with_access(
        server_admin_contract::domain_types::AdminPage::Settings,
        content,
        Some(admin),
        Some(branding),
    )
}
