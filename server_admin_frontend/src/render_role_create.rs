#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "the server-rendered CRUD forms require Leptos attribute traits after macro expansion"
)]

#[allow(
    unused_import_braces,
    reason = "grouped Leptos prelude imports are required by workspace source policy"
)]
#[rustfmt::skip]
use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes};

#[must_use]
pub fn render_role_create(
    admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    branding: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    super::crud_render_shell::crud_render_shell(
        server_admin_contract::admin_page::AdminPage::Roles,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">"Roles"</p><h1>"Create role"</h1><p>"Create a role before assigning its permissions."</p></div><crate::admin_button_link::AdminButtonLink href=server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles.get() variant=crate::admin_button_variant::AdminButtonVariant::Secondary>"Back to roles"</crate::admin_button_link::AdminButtonLink></div>
            <crate::admin_card::AdminCard><form class="crud-form" method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::RoleCreate.get()>
                <crate::admin_field::AdminField label="Role name"><crate::admin_input::AdminInput name="name" required=true /></crate::admin_field::AdminField>
                <div class="crud-actions"><crate::admin_button::AdminButton>"Create role"</crate::admin_button::AdminButton></div>
            </form></crate::admin_card::AdminCard></section>
        },
        admin,
        branding,
    )
}
