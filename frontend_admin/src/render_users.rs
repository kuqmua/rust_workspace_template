use leptos::prelude::{ClassAttribute, ElementChild};

#[must_use]
pub fn render_users(
    admin_users_page: &server_admin_contract::admin_users_page::AdminUsersPage,
    admin_table_query: &server_admin_contract::admin_table_query::AdminTableQuery,
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let can_create = bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::UsersCreate),
    );
    let can_manage = bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::UsersUpdate),
    ) || bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::UsersDelete),
    );
    let rows = admin_users_page
        .items()
        .iter()
        .map(|item| {
            let id = item.id().to_string();
            let login = item.login().to_string();
            let display_name = item.display_name().to_string();
            let banned = item.is_banned().to_string();
            let roles =
                crate::admin_user_roles::admin_user_roles(item, admin_users_page);
            leptos::view! {
                <crate::table_row::TableRow>
                    <crate::table_cell::TableCell data_label="id">{id}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="login">{login}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="display_name">{display_name}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="banned">{banned}</crate::table_cell::TableCell>
                    {roles}
                </crate::table_row::TableRow>
            }
        })
        .collect::<Vec<_>>();
    let content_view = leptos::view! {
        <section class="table-page table-admin_users_page">
        <div class="resource-actions">
            {can_create.then(|| leptos::view! { <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersCreate.get()>"Create user"</crate::admin_button_link::AdminButtonLink> })}
            {can_manage.then(|| leptos::view! { <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersManage.get() admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary>"Manage users"</crate::admin_button_link::AdminButtonLink> })}
        </div>
        <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead>"id"</crate::table_head::TableHead><crate::table_head::TableHead>"login"</crate::table_head::TableHead><crate::table_head::TableHead>"display_name"</crate::table_head::TableHead><crate::table_head::TableHead>"banned"</crate::table_head::TableHead><crate::table_head::TableHead>"roles"</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
        <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
        {crate::table_pagination::table_pagination(server_admin_contract::admin_page::AdminPage::Users, admin_table_query, admin_users_page.total(), None, None)}
        </section>
    };
    let content = crate::render_view::render_view(content_view);
    crate::render_admin_page_with_access::render_admin_page_with_access(
        server_admin_contract::admin_page::AdminPage::Users,
        content,
        Some(authenticated_admin),
        Some(admin_branding_view),
    )
}
