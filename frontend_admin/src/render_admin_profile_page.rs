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
pub fn render_admin_profile_page(
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let roles = String::from(crate::join_text::join_text(
        authenticated_admin
            .roles()
            .iter()
            .map(|name| name.as_ref().as_str()),
    ));
    let display_name = authenticated_admin.display_name().to_string();
    let login = authenticated_admin.login().to_string();
    let content_view = leptos::view! {
        <crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Profile><crate::admin_card_header::AdminCardHeader><crate::admin_card_title::AdminCardTitle>{display_name}</crate::admin_card_title::AdminCardTitle><crate::admin_card_description::AdminCardDescription>{login}</crate::admin_card_description::AdminCardDescription></crate::admin_card_header::AdminCardHeader><p>{roles}</p></crate::admin_card::AdminCard>
        <crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Security><form method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::ProfilePassword.get()>
            <crate::admin_field::AdminField admin_field_label="Current password"><crate::admin_input::AdminInput admin_input_name="current_password" admin_input_kind=crate::admin_input_kind::AdminInputKind::Password required=true /></crate::admin_field::AdminField>
            <crate::admin_field::AdminField admin_field_label="New password"><crate::admin_input::AdminInput admin_input_name="new_password" admin_input_kind=crate::admin_input_kind::AdminInputKind::Password minlength=server_admin_contract::identity::ADMIN_NEW_PASSWORD_MIN_CHARS maxlength=server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS required=true /><singlestage::FieldDescription attr:class="password-policy">{constants_str::ADMIN_PASSWORD_POLICY_DESCRIPTION}</singlestage::FieldDescription></crate::admin_field::AdminField>
            <crate::admin_card_footer::AdminCardFooter><crate::admin_button::AdminButton>{constants_str::ADMIN_BUTTON_CHANGE_PASSWORD}</crate::admin_button::AdminButton></crate::admin_card_footer::AdminCardFooter>
        </form></crate::admin_card::AdminCard>
    };
    let content = crate::render_view::render_view(content_view);
    crate::render_admin_page_with_access::render_admin_page_with_access(
        server_admin_contract::admin_page::AdminPage::Profile,
        content,
        Some(authenticated_admin),
        Some(admin_branding_view),
    )
}
