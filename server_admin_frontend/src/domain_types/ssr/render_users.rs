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
        <crate::domain_types::with_owner::table::TableWrapper><crate::domain_types::with_owner::table::Table><crate::domain_types::with_owner::table::TableHeader><crate::domain_types::with_owner::table::TableRow><crate::domain_types::with_owner::table::TableHead>"id"</crate::domain_types::with_owner::table::TableHead><crate::domain_types::with_owner::table::TableHead>"login"</crate::domain_types::with_owner::table::TableHead><crate::domain_types::with_owner::table::TableHead>"display_name"</crate::domain_types::with_owner::table::TableHead><crate::domain_types::with_owner::table::TableHead>"banned"</crate::domain_types::with_owner::table::TableHead><crate::domain_types::with_owner::table::TableHead>"roles"</crate::domain_types::with_owner::table::TableHead></crate::domain_types::with_owner::table::TableRow></crate::domain_types::with_owner::table::TableHeader>
        <crate::domain_types::with_owner::table::TableBody>{rows}</crate::domain_types::with_owner::table::TableBody></crate::domain_types::with_owner::table::Table></crate::domain_types::with_owner::table::TableWrapper>
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
