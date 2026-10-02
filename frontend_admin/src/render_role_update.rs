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
pub fn render_role_update(
    admin_role_id: Option<server_admin_contract::admin_role_id::AdminRoleId>,
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let action = admin_role_id.map_or_else(
        || {
            server_admin_contract::admin_html_action::AdminHtmlAction::RoleUpdate
                .get()
                .to_owned()
        },
        |admin_role_id| {
            server_admin_contract::admin_route_path::AdminRoutePath::role_update_action_path(
                admin_role_id,
            )
            .to_string()
        },
    );
    let identifier = admin_role_id.map_or_else(
        || leptos::prelude::IntoAny::into_any(leptos::view! {
            <crate::admin_field::AdminField admin_field_label=constants_str::ROLE_ID><crate::admin_input::AdminInput admin_input_name=constants_str::ROLE_ID admin_input_kind=crate::admin_input_kind::AdminInputKind::Number min=1u16 required=true /></crate::admin_field::AdminField>
        }),
        |admin_role_id| leptos::prelude::IntoAny::into_any(leptos::view! {
            <input type="hidden" name=constants_str::ROLE_ID value=admin_role_id.to_string() />
        }),
    );
    super::crud_render_shell::crud_render_shell(
        server_admin_contract::admin_page::AdminPage::Roles,
        leptos::view! {
            <section class="crud-page role-update-page">
                <form class="role-update-form" method="post" action=action>
                    <section class="crud-form">
                        {identifier}
                        <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_ROLE_NAME><crate::admin_input::AdminInput admin_input_name="name" required=true /></crate::admin_field::AdminField>
                    </section>
                    <div class="crud-actions"><crate::admin_button::AdminButton>{constants_str::PG_CRUD_UPDATE_RULE_ACTION}</crate::admin_button::AdminButton></div>
                </form>
            </section>
        },
        authenticated_admin,
        admin_branding_view,
    )
}
