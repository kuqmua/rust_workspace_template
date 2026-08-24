mod row;

use leptos::prelude::{ClassAttribute, ElementChild};

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
    let can_create =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesCreate));
    let can_manage =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesUpdate))
            || bool::from(
                admin.has_permission(server_admin_contract::AdminPermission::RolesDelete),
            );
    let rows = page
        .items()
        .iter()
        .map(|item| row::admin_role_row(item, page))
        .collect::<Vec<_>>();
    let content_view = leptos::view! {
        <section class="table-page">
        <div class="resource-actions">
            {can_create.then(|| leptos::view! { <crate::ui::button::AdminButtonLink href=server_admin_contract::AdminFrontendPath::RolesCreate.get()>"Create role"</crate::ui::button::AdminButtonLink> })}
            {can_manage.then(|| leptos::view! { <crate::ui::button::AdminButtonLink href=server_admin_contract::AdminFrontendPath::RolesManage.get() variant=crate::ui::button::AdminButtonVariant::Secondary>"Manage roles"</crate::ui::button::AdminButtonLink> })}
        </div>
        <crate::ui::table::TableWrapper><crate::ui::table::Table><crate::ui::table::TableHeader><crate::ui::table::TableRow><crate::ui::table::TableHead>"id"</crate::ui::table::TableHead><crate::ui::table::TableHead>"name"</crate::ui::table::TableHead><crate::ui::table::TableHead>"system"</crate::ui::table::TableHead><crate::ui::table::TableHead>"permissions"</crate::ui::table::TableHead></crate::ui::table::TableRow></crate::ui::table::TableHeader>
        <crate::ui::table::TableBody>{rows}</crate::ui::table::TableBody></crate::ui::table::Table></crate::ui::table::TableWrapper>
        {super::table_pagination(server_admin_contract::AdminPage::Roles, query, page.total(), None, None)}
        </section>
    };
    let content = super::render_view(content_view);
    super::render_admin_page_with_access(
        server_admin_contract::AdminPage::Roles,
        content,
        Some(admin),
        Some(branding),
    )
}
