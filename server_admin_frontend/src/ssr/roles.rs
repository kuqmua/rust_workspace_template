mod create;
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
    page: &server_admin_contract::AdminRolesPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let can_create = admin.has_permission(server_admin_contract::AdminPermission::RolesCreate);
    let can_delete = admin.has_permission(server_admin_contract::AdminPermission::RolesDelete);
    let can_update = admin.has_permission(server_admin_contract::AdminPermission::RolesUpdate);
    let can_update_permissions =
        admin.has_permission(server_admin_contract::AdminPermission::RolePermissionsUpdate);
    let content = leptos::view! {
        <section class="table-page">
        {crate::shared::table_filters::form::admin_table_filters(server_admin_contract::AdminFrontendPath::Roles, query.search(), query.sort(), crate::shared::table_filters::form::AdminTableFilterDirection::from(query.direction()), query.limit(), &server_admin_contract::AdminTableSortField::ROLE, crate::shared::table_filters::form::AdminTableFilterPresentation::Ssr)}
        {create::admin_create_role(can_create)}
        <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"name"</th><th>"system"</th><th>"permissions"</th><th>"actions"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| row::admin_role_row(item, page, can_delete, can_update, can_update_permissions)).collect::<Vec<_>>()}</tbody></table></div>
        {super::table_pagination(server_admin_contract::AdminPage::Roles, query, page.total(), None, None)}
        </section>
    }.render_admin_ssr();
    super::render_admin_page_with_access(
        server_admin_contract::AdminPage::Roles,
        content,
        Some(admin),
        Some(branding),
    )
}
