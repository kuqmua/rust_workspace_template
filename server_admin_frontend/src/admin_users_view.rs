use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[path = "domain_types_start_admin_users_view_admin_user_row.rs"]
mod admin_user_row;

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
pub(in crate::domain_types::start) fn AdminUsersView(
    admin: server_admin_contract::domain_types::AuthenticatedAdmin,
    page: server_admin_contract::domain_types::AdminUsersPage,
    query: super::query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let can_create = bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::UsersCreate),
    );
    let can_manage = bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::UsersUpdate),
    ) || bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::UsersDelete),
    );
    let total = page.total();
    let rows = page
        .items()
        .iter()
        .map(|item| admin_user_row::admin_user_row(item, &page))
        .collect::<Vec<_>>();
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <div class="resource-actions">
                {can_create.then(|| leptos::view! { <crate::domain_types::with_owner::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::UsersCreate.get()>"Create user"</crate::domain_types::with_owner::button::AdminButtonLink> })}
                {can_manage.then(|| leptos::view! { <crate::domain_types::with_owner::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::UsersManage.get() variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary>"Manage users"</crate::domain_types::with_owner::button::AdminButtonLink> })}
            </div>
            <crate::domain_types::with_owner::table::TableWrapper><crate::domain_types::with_owner::table::Table><crate::domain_types::with_owner::table::TableHeader><crate::domain_types::with_owner::table::TableRow><crate::domain_types::with_owner::table::TableHead>"id"</crate::domain_types::with_owner::table::TableHead><crate::domain_types::with_owner::table::TableHead>"login"</crate::domain_types::with_owner::table::TableHead><crate::domain_types::with_owner::table::TableHead>"display_name"</crate::domain_types::with_owner::table::TableHead><crate::domain_types::with_owner::table::TableHead>"banned"</crate::domain_types::with_owner::table::TableHead><crate::domain_types::with_owner::table::TableHead>"roles"</crate::domain_types::with_owner::table::TableHead></crate::domain_types::with_owner::table::TableRow></crate::domain_types::with_owner::table::TableHeader>
            <crate::domain_types::with_owner::table::TableBody>{rows}</crate::domain_types::with_owner::table::TableBody></crate::domain_types::with_owner::table::Table></crate::domain_types::with_owner::table::TableWrapper>
            <super::pagination::AdminPagination action=server_admin_contract::domain_types::AdminFrontendPath::Users query=query total=total />
        </section>
    }
}
