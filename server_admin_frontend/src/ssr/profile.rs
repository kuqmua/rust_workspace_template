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
    let display_name = admin.display_name().to_string();
    let login = admin.login().to_string();
    let content_view = leptos::view! {
        <crate::ui::card::AdminCard variant=crate::ui::card::AdminCardVariant::Profile><p><strong>{display_name}</strong></p><p>{login}</p><p>{roles}</p></crate::ui::card::AdminCard>
        <crate::ui::card::AdminCard variant=crate::ui::card::AdminCardVariant::Security><form method="post" action=server_admin_contract::AdminHtmlAction::ProfilePassword.get()>
            <p class="password-policy">{str_constants::ADMIN_PASSWORD_POLICY_DESCRIPTION}</p>
            <crate::ui::field::AdminField label="Current password"><crate::ui::input::AdminInput name="current_password" kind=crate::ui::input::AdminInputKind::Password required=true /></crate::ui::field::AdminField>
            <crate::ui::field::AdminField label="New password"><crate::ui::input::AdminInput name="new_password" kind=crate::ui::input::AdminInputKind::Password minlength=server_admin_contract::ADMIN_NEW_PASSWORD_MIN_CHARS maxlength=server_admin_contract::ADMIN_PASSWORD_MAX_CHARS required=true /></crate::ui::field::AdminField>
            <crate::ui::button::AdminButton>"Change password"</crate::ui::button::AdminButton>
        </form></crate::ui::card::AdminCard>
    };
    let content = super::render_view(content_view);
    super::render_admin_page_with_access(
        server_admin_contract::AdminPage::Profile,
        content,
        Some(admin),
        Some(branding),
    )
}
