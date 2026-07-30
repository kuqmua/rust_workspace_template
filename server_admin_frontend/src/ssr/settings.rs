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

trait AdminSsrViewExt {
    fn render_admin_ssr(self) -> super::AdminSsrHtml;
}
impl<View> AdminSsrViewExt for View
where
    View: leptos::prelude::IntoAny,
{
    fn render_admin_ssr(self) -> super::AdminSsrHtml {
        super::AdminSsrHtml::try_from(leptos::prelude::RenderHtml::to_html(
            leptos::prelude::IntoAny::into_any(self),
        ))
        .unwrap_or_else(super::AdminSsrHtml::from)
    }
}

#[must_use]
pub(super) fn render_settings(
    view: &server_admin_contract::AdminSettingsView,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let values = crate::shared::settings::values::AdminSettingsFormValues::from(view);
    let signals = crate::shared::settings::signals::AdminSettingsFormSignals::new(&values);
    let can_update = bool::from(
        admin.has_permission(server_admin_contract::AdminPermission::SystemSettingsUpdate),
    );
    let content = leptos::view! {
        <section class="settings-grid"><article class="settings-card">
        {can_update.then(|| leptos::view! { <form class="settings-form" method="post" action=server_admin_contract::AdminHtmlAction::SettingsUpdate.get()>
            {crate::shared::settings::input::admin_setting_inputs(signals, crate::shared::settings::input::AdminSettingDisabled::from(false))}
            <button type="submit">"Save settings"</button>
        </form> })}
        {(!can_update).then(|| leptos::view! { <p>"Settings are read-only for this account."</p> })}
        </article></section>
    }.render_admin_ssr();
    super::render_admin_page_with_access(
        server_admin_contract::AdminPage::Settings,
        content,
        Some(admin),
        Some(branding),
    )
}
