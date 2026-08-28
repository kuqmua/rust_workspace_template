use leptos::prelude::{ClassAttribute, ElementChild};

#[must_use]
pub fn render_users(
    page: &server_admin_contract::domain_types::AdminUsersPage,
    query: &server_admin_contract::domain_types::AdminTableQuery,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
    let can_create = bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::UsersCreate),
    );
    let can_manage = bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::UsersUpdate),
    ) || bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::UsersDelete),
    );
    let rows = page
        .items()
        .iter()
        .map(|item| {
            let id = item.id().to_string();
            let login = item.login().to_string();
            let display_name = item.display_name().to_string();
            let banned = item.is_banned().to_string();
            let roles =
                crate::domain_types::shared::admin_user_roles::admin_user_roles(item, page);
            leptos::view! {
                <crate::domain_types::with_owner::tables::table_row::TableRow>
                    <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="id">{id}</crate::domain_types::with_owner::tables::table_cell::TableCell>
                    <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="login">{login}</crate::domain_types::with_owner::tables::table_cell::TableCell>
                    <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="display_name">{display_name}</crate::domain_types::with_owner::tables::table_cell::TableCell>
                    <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="banned">{banned}</crate::domain_types::with_owner::tables::table_cell::TableCell>
                    {roles}
                </crate::domain_types::with_owner::tables::table_row::TableRow>
            }
        })
        .collect::<Vec<_>>();
    let content_view = leptos::view! {
        <section class="table-page">
        <div class="resource-actions">
            {can_create.then(|| leptos::view! { <crate::domain_types::with_owner::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::UsersCreate.get()>"Create user"</crate::domain_types::with_owner::button::AdminButtonLink> })}
            {can_manage.then(|| leptos::view! { <crate::domain_types::with_owner::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::UsersManage.get() variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary>"Manage users"</crate::domain_types::with_owner::button::AdminButtonLink> })}
        </div>
        <crate::domain_types::with_owner::tables::table_wrapper::TableWrapper><crate::domain_types::with_owner::tables::table::Table><crate::domain_types::with_owner::tables::table_header::TableHeader><crate::domain_types::with_owner::tables::table_row::TableRow><crate::domain_types::with_owner::tables::table_head::TableHead>"id"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"login"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"display_name"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"banned"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"roles"</crate::domain_types::with_owner::tables::table_head::TableHead></crate::domain_types::with_owner::tables::table_row::TableRow></crate::domain_types::with_owner::tables::table_header::TableHeader>
        <crate::domain_types::with_owner::tables::table_body::TableBody>{rows}</crate::domain_types::with_owner::tables::table_body::TableBody></crate::domain_types::with_owner::tables::table::Table></crate::domain_types::with_owner::tables::table_wrapper::TableWrapper>
        {super::table_pagination(server_admin_contract::domain_types::AdminPage::Users, query, page.total(), None, None)}
        </section>
    };
    let content = super::render_view(content_view);
    super::render_admin_page_with_access(
        server_admin_contract::domain_types::AdminPage::Users,
        content,
        Some(admin),
        Some(branding),
    )
}
