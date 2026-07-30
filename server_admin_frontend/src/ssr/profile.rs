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
pub(super) fn render_profile(
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let roles = admin
        .roles()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(str_constants::COMMA_SPACE);
    let content = leptos::view! {
        <section class="security-card"><p><strong>{admin.display_name().to_string()}</strong></p><p>{admin.login().to_string()}</p><p>{roles}</p></section>
        <section class="security-card"><form method="post" action=server_admin_contract::AdminHtmlAction::ProfilePassword.get()>
            <p class="password-policy">{str_constants::ADMIN_PASSWORD_POLICY_DESCRIPTION}</p>
            <label><span>"Current password"</span><input name="current_password" type="password" required /></label>
            <label><span>"New password"</span><input name="new_password" type="password" minlength=server_admin_contract::ADMIN_NEW_PASSWORD_MIN_CHARS maxlength=server_admin_contract::ADMIN_PASSWORD_MAX_CHARS required /></label>
            <button type="submit">"Change password"</button>
        </form></section>
    }.render_admin_ssr();
    super::render_admin_page_with_access(
        server_admin_contract::AdminPage::Profile,
        content,
        Some(admin),
        Some(branding),
    )
}
