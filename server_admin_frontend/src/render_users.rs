#[path = "domain_types_ssr_render_users_admin_user_row.rs"]
mod admin_user_row;

use leptos::prelude::{ClassAttribute, ElementChild};

#[allow(
    clippy::single_call_fn,
    reason = "the screen renderer is isolated behind the stable public SSR facade"
)]
pub(super) fn render_users(
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
        .map(|item| admin_user_row::admin_user_row(item, page))
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
