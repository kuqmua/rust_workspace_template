mod row;

use leptos::prelude::{ClassAttribute, ElementChild};

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

#[allow(
    clippy::single_call_fn,
    reason = "the screen renderer is isolated behind the stable public SSR facade"
)]
pub(super) fn render(
    page: &server_admin_contract::AdminUsersPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let can_delete = admin.has_permission(server_admin_contract::AdminPermission::UsersDelete);
    let can_update = admin.has_permission(server_admin_contract::AdminPermission::UsersUpdate);
    let can_update_roles =
        admin.has_permission(server_admin_contract::AdminPermission::UserRolesUpdate);
    let content = leptos::view! {
        <section class="table-page">
        <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"login"</th><th>"display_name"</th><th>"banned"</th><th>"roles"</th><th>"actions"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| row::admin_user_row(item, page, can_delete, can_update, can_update_roles)).collect::<Vec<_>>()}</tbody></table></div>
        {super::table_pagination(server_admin_contract::AdminPage::Users, query, page.total(), None, None)}
        </section>
    }.render_admin_ssr();
    super::render_admin_page_with_access(
        server_admin_contract::AdminPage::Users,
        content,
        Some(admin),
        Some(branding),
    )
}
