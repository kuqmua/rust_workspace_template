use leptos::prelude::{ClassAttribute, ElementChild};

#[must_use]
pub fn render_roles(
    admin_roles_page: &server_admin_contract::admin_roles_page::AdminRolesPage,
    admin_table_query: &server_admin_contract::admin_table_query::AdminTableQuery,
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let can_create = bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::RolesCreate),
    );
    let can_manage = bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::RolesUpdate),
    ) || bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::RolesDelete),
    );
    let rows = admin_roles_page
        .items()
        .iter()
        .map(|item| {
            let id = item.id().to_string();
            let name = item.name().to_string();
            let system = item.is_system().to_string();
            let permissions =
                crate::admin_role_permissions::admin_role_permissions(
                    item, admin_roles_page,
                );
            leptos::view! {
                <crate::table_row::TableRow>
                    <crate::table_cell::TableCell data_label="id">{id}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="name">{name}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="system">{system}</crate::table_cell::TableCell>
                    {permissions}
                </crate::table_row::TableRow>
            }
        })
        .collect::<Vec<_>>();
    let content_view = leptos::view! {
        <section class="table-page">
        <div class="resource-actions">
            {can_create.then(|| leptos::view! { <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::RolesCreate.get()>"Create role"</crate::admin_button_link::AdminButtonLink> })}
            {can_manage.then(|| leptos::view! { <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::RolesManage.get() admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary>"Manage roles"</crate::admin_button_link::AdminButtonLink> })}
        </div>
        <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead>"id"</crate::table_head::TableHead><crate::table_head::TableHead>"name"</crate::table_head::TableHead><crate::table_head::TableHead>"system"</crate::table_head::TableHead><crate::table_head::TableHead>"permissions"</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
        <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
        {crate::table_pagination::table_pagination(server_admin_contract::admin_page::AdminPage::Roles, admin_table_query, admin_roles_page.total(), None, None)}
        </section>
    };
    let content = crate::render_view::render_view(content_view);
    crate::render_admin_page_with_access::render_admin_page_with_access(
        server_admin_contract::admin_page::AdminPage::Roles,
        content,
        Some(authenticated_admin),
        Some(admin_branding_view),
    )
}
