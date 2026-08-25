mod row;

use leptos::prelude::{ClassAttribute, ElementChild};

#[allow(
    clippy::single_call_fn,
    reason = "the screen renderer is isolated behind the stable public SSR facade"
)]
pub(super) fn render(
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
        .map(|item| row::admin_role_row(item, page))
        .collect::<Vec<_>>();
    let content_view = leptos::view! {
        <section class="table-page">
        <div class="resource-actions">
            {can_create.then(|| leptos::view! { <crate::domain_types::ui::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::RolesCreate.get()>"Create role"</crate::domain_types::ui::button::AdminButtonLink> })}
            {can_manage.then(|| leptos::view! { <crate::domain_types::ui::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::RolesManage.get() variant=crate::domain_types::ui::button::AdminButtonVariant::Secondary>"Manage roles"</crate::domain_types::ui::button::AdminButtonLink> })}
        </div>
        <crate::domain_types::ui::table::TableWrapper><crate::domain_types::ui::table::Table><crate::domain_types::ui::table::TableHeader><crate::domain_types::ui::table::TableRow><crate::domain_types::ui::table::TableHead>"id"</crate::domain_types::ui::table::TableHead><crate::domain_types::ui::table::TableHead>"name"</crate::domain_types::ui::table::TableHead><crate::domain_types::ui::table::TableHead>"system"</crate::domain_types::ui::table::TableHead><crate::domain_types::ui::table::TableHead>"permissions"</crate::domain_types::ui::table::TableHead></crate::domain_types::ui::table::TableRow></crate::domain_types::ui::table::TableHeader>
        <crate::domain_types::ui::table::TableBody>{rows}</crate::domain_types::ui::table::TableBody></crate::domain_types::ui::table::Table></crate::domain_types::ui::table::TableWrapper>
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
