#![allow(
    clippy::unused_trait_names,
    reason = "the server-rendered CRUD forms require Leptos attribute traits after macro expansion"
)]

#[allow(
    unused_import_braces,
    reason = "grouped Leptos prelude imports are required by workspace source policy"
)]
#[rustfmt::skip]
use leptos::prelude::{ClassAttribute, ElementChild};

#[must_use]
pub fn render_role_create(
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    super::crud_render_shell::crud_render_shell(
        server_admin_contract::admin_page::AdminPage::Roles,
        leptos::view! {
            <section class="crud-page role-create-page">
            <form class="role-create-form" method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::RoleCreate.get()><section class="crud-form">
                <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_ROLE_NAME><crate::admin_input::AdminInput admin_input_name="name" required=true /></crate::admin_field::AdminField>
            </section>
                <div class="crud-actions"><crate::admin_button::AdminButton>{constants_str::ADMIN_BUTTON_CREATE_ROLE}</crate::admin_button::AdminButton></div>
            </form></section>
        },
        authenticated_admin,
        admin_branding_view,
    )
}
