#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

#[must_use]
#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(super) fn render_profile(
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
    let roles = String::from(crate::domain_types::shared::text::join_text(
        admin.roles().iter().map(|name| name.as_ref().as_str()),
    ));
    let display_name = admin.display_name().to_string();
    let login = admin.login().to_string();
    let content_view = leptos::view! {
        <crate::domain_types::with_owner::card::AdminCard variant=crate::domain_types::with_owner::card::AdminCardVariant::Profile><crate::domain_types::with_owner::card::AdminCardHeader><crate::domain_types::with_owner::card::AdminCardTitle>{display_name}</crate::domain_types::with_owner::card::AdminCardTitle><crate::domain_types::with_owner::card::AdminCardDescription>{login}</crate::domain_types::with_owner::card::AdminCardDescription></crate::domain_types::with_owner::card::AdminCardHeader><p>{roles}</p></crate::domain_types::with_owner::card::AdminCard>
        <crate::domain_types::with_owner::card::AdminCard variant=crate::domain_types::with_owner::card::AdminCardVariant::Security><form method="post" action=server_admin_contract::domain_types::AdminHtmlAction::ProfilePassword.get()>
            <crate::domain_types::with_owner::field::AdminField label="Current password"><crate::domain_types::with_owner::input::AdminInput name="current_password" kind=crate::domain_types::with_owner::input::AdminInputKind::Password required=true /></crate::domain_types::with_owner::field::AdminField>
            <crate::domain_types::with_owner::field::AdminField label="New password"><crate::domain_types::with_owner::input::AdminInput name="new_password" kind=crate::domain_types::with_owner::input::AdminInputKind::Password minlength=server_admin_contract::domain_types::ADMIN_NEW_PASSWORD_MIN_CHARS maxlength=server_admin_contract::domain_types::ADMIN_PASSWORD_MAX_CHARS required=true /><singlestage::FieldDescription attr:class="password-policy">{constants_str::ADMIN_PASSWORD_POLICY_DESCRIPTION}</singlestage::FieldDescription></crate::domain_types::with_owner::field::AdminField>
            <crate::domain_types::with_owner::card::AdminCardFooter><crate::domain_types::with_owner::button::AdminButton>"Change password"</crate::domain_types::with_owner::button::AdminButton></crate::domain_types::with_owner::card::AdminCardFooter>
        </form></crate::domain_types::with_owner::card::AdminCard>
    };
    let content = super::render_view(content_view);
    super::render_admin_page_with_access(
        server_admin_contract::domain_types::AdminPage::Profile,
        content,
        Some(admin),
        Some(branding),
    )
}
