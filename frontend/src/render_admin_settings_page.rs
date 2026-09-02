#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

#[must_use]
pub fn render_admin_settings_page(
    admin_settings_view: &server_admin_contract::admin_settings_view::AdminSettingsView,
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let values =
        crate::admin_settings_form_values::AdminSettingsFormValues::from(admin_settings_view);
    let signals = crate::admin_settings_form_signals::AdminSettingsFormSignals::new(&values);
    let can_update = bool::from(authenticated_admin.has_permission(
        server_admin_contract::admin_permission::AdminPermission::SystemSettingsUpdate,
    ));
    let content_view = leptos::view! {
        <section class="settings-grid"><crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Settings>
        {can_update.then(|| leptos::view! { <form class="settings-form" method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::SettingsUpdate.get()>
            {crate::admin_setting_inputs::admin_setting_inputs(signals, crate::admin_setting_disabled::AdminSettingDisabled::from(false))}
            <crate::admin_card_footer::AdminCardFooter><crate::admin_button::AdminButton>"Save settings"</crate::admin_button::AdminButton></crate::admin_card_footer::AdminCardFooter>
        </form> })}
        {(!can_update).then(|| leptos::view! { <crate::admin_alert::AdminAlert>"Settings are read-only for this account."</crate::admin_alert::AdminAlert> })}
        </crate::admin_card::AdminCard></section>
    };
    let content = crate::render_view::render_view(content_view);
    crate::render_admin_page_with_access::render_admin_page_with_access(
        server_admin_contract::admin_page::AdminPage::Settings,
        content,
        Some(authenticated_admin),
        Some(admin_branding_view),
    )
}
