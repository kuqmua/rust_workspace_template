use leptos::prelude::{ClassAttribute, ElementChild};

#[must_use]
pub fn render_roles(
    page: &server_admin_contract::domain_types::AdminRolesPage,
    query: &server_admin_contract::domain_types::AdminTableQuery,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
    let can_create = bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::RolesCreate),
    );
    let can_manage = bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::RolesUpdate),
    ) || bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::RolesDelete),
    );
    let rows = page
        .items()
        .iter()
        .map(|item| {
            let id = item.id().to_string();
            let name = item.name().to_string();
            let system = item.is_system().to_string();
            let permissions =
                crate::domain_types::shared::admin_role_permissions::admin_role_permissions(
                    item, page,
                );
            leptos::view! {
                <crate::domain_types::with_owner::tables::table_row::TableRow>
                    <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="id">{id}</crate::domain_types::with_owner::tables::table_cell::TableCell>
                    <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="name">{name}</crate::domain_types::with_owner::tables::table_cell::TableCell>
                    <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="system">{system}</crate::domain_types::with_owner::tables::table_cell::TableCell>
                    {permissions}
                </crate::domain_types::with_owner::tables::table_row::TableRow>
            }
        })
        .collect::<Vec<_>>();
    let content_view = leptos::view! {
        <section class="table-page">
        <div class="resource-actions">
            {can_create.then(|| leptos::view! { <crate::domain_types::with_owner::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::RolesCreate.get()>"Create role"</crate::domain_types::with_owner::button::AdminButtonLink> })}
            {can_manage.then(|| leptos::view! { <crate::domain_types::with_owner::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::RolesManage.get() variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary>"Manage roles"</crate::domain_types::with_owner::button::AdminButtonLink> })}
        </div>
        <crate::domain_types::with_owner::tables::table_wrapper::TableWrapper><crate::domain_types::with_owner::tables::table::Table><crate::domain_types::with_owner::tables::table_header::TableHeader><crate::domain_types::with_owner::tables::table_row::TableRow><crate::domain_types::with_owner::tables::table_head::TableHead>"id"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"name"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"system"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"permissions"</crate::domain_types::with_owner::tables::table_head::TableHead></crate::domain_types::with_owner::tables::table_row::TableRow></crate::domain_types::with_owner::tables::table_header::TableHeader>
        <crate::domain_types::with_owner::tables::table_body::TableBody>{rows}</crate::domain_types::with_owner::tables::table_body::TableBody></crate::domain_types::with_owner::tables::table::Table></crate::domain_types::with_owner::tables::table_wrapper::TableWrapper>
        {super::table_pagination(server_admin_contract::domain_types::AdminPage::Roles, query, page.total(), None, None)}
        </section>
    };
    let content = super::render_view(content_view);
    super::render_admin_page_with_access(
        server_admin_contract::domain_types::AdminPage::Roles,
        content,
        Some(admin),
        Some(branding),
    )
}
