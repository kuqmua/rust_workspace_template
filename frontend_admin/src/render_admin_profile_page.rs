#![allow(
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::ElementChild;

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
    };
    let content = crate::render_view::render_view(content_view);
    crate::render_admin_page_with_access::render_admin_page_with_access(
        server_admin_contract::admin_page::AdminPage::Profile,
        content,
        Some(authenticated_admin),
        Some(admin_branding_view),
    )
}
