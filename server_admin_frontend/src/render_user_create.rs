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
pub fn render_user_create(
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    super::crud_render_shell::crud_render_shell(
        server_admin_contract::admin_page::AdminPage::Users,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">"Users"</p><h1>"Create user"</h1><p>"Add a user account with initial credentials."</p></div><crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get() admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary>"Back to users"</crate::admin_button_link::AdminButtonLink></div>
            <crate::admin_card::AdminCard><form class="crud-form" method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::UserCreate.get()>
                <crate::admin_field::AdminField admin_field_label="Login"><crate::admin_input::AdminInput admin_input_name="login" autocomplete="username" required=true /></crate::admin_field::AdminField>
                <crate::admin_field::AdminField admin_field_label="Display name"><crate::admin_input::AdminInput admin_input_name="display_name" required=true /></crate::admin_field::AdminField>
                <crate::admin_field::AdminField admin_field_label="Initial password"><crate::admin_input::AdminInput admin_input_name="password" admin_input_kind=crate::admin_input_kind::AdminInputKind::Password autocomplete="new-password" required=true /></crate::admin_field::AdminField>
                <div class="crud-actions"><crate::admin_button::AdminButton>"Create user"</crate::admin_button::AdminButton></div>
            </form></crate::admin_card::AdminCard></section>
        },
        authenticated_admin,
        admin_branding_view,
    )
}
