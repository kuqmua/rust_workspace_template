#![allow(
    clippy::unused_trait_names,
    reason = "the server-rendered CRUD form requires Leptos attribute traits after macro expansion"
)]

#[allow(
    unused_import_braces,
    reason = "grouped Leptos prelude imports are required by workspace source policy"
)]
#[rustfmt::skip]
use leptos::prelude::{ClassAttribute, ElementChild};

#[must_use]
pub fn render_user_update(
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    super::crud_render_shell::crud_render_shell(
        server_admin_contract::admin_page::AdminPage::Users,
        leptos::view! {
            <section class="crud-page user-update-page">
                <form class="user-update-form" method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::UserUpdate.get()>
                    <section class="crud-form">
                        <crate::admin_field::AdminField admin_field_label=constants_str::USER_ID><crate::admin_input::AdminInput admin_input_name=constants_str::USER_ID admin_input_kind=crate::admin_input_kind::AdminInputKind::Number min=1u16 required=true /></crate::admin_field::AdminField>
                        <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_DISPLAY_NAME><crate::admin_input::AdminInput admin_input_name="display_name" required=true /></crate::admin_field::AdminField>
                        <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_LOGIN><crate::admin_input::AdminInput admin_input_name="login" autocomplete="username" required=true /></crate::admin_field::AdminField>
                    </section>
                    <div class="crud-actions"><crate::admin_button::AdminButton>{constants_str::PG_CRUD_UPDATE_RULE_ACTION}</crate::admin_button::AdminButton></div>
                </form>
            </section>
        },
        authenticated_admin,
        admin_branding_view,
    )
}
