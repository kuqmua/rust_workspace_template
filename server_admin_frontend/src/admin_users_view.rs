use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
pub(crate) fn AdminUsersView(
    admin: server_admin_contract::domain_types::AuthenticatedAdmin,
    page: server_admin_contract::domain_types::AdminUsersPage,
    query: super::admin_csr_query::AdminCsrQuery,
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
        .map(|item| csr_admin_user_row::csr_admin_user_row(item, &page))
        .collect::<Vec<_>>();
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <div class="resource-actions">
                {can_create.then(|| leptos::view! { <crate::domain_types::with_owner::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::UsersCreate.get()>"Create user"</crate::domain_types::with_owner::button::AdminButtonLink> })}
                {can_manage.then(|| leptos::view! { <crate::domain_types::with_owner::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::UsersManage.get() variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary>"Manage users"</crate::domain_types::with_owner::button::AdminButtonLink> })}
            </div>
            <crate::domain_types::with_owner::tables::table_wrapper::TableWrapper><crate::domain_types::with_owner::tables::table::Table><crate::domain_types::with_owner::tables::table_header::TableHeader><crate::domain_types::with_owner::tables::table_row::TableRow><crate::domain_types::with_owner::tables::table_head::TableHead>"id"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"login"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"display_name"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"banned"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"roles"</crate::domain_types::with_owner::tables::table_head::TableHead></crate::domain_types::with_owner::tables::table_row::TableRow></crate::domain_types::with_owner::tables::table_header::TableHeader>
            <crate::domain_types::with_owner::tables::table_body::TableBody>{rows}</crate::domain_types::with_owner::tables::table_body::TableBody></crate::domain_types::with_owner::tables::table::Table></crate::domain_types::with_owner::tables::table_wrapper::TableWrapper>
            <super::admin_pagination::AdminPagination action=server_admin_contract::domain_types::AdminFrontendPath::Users query=query total=total />
        </section>
    }
}

// Root-owned module compatibility wrappers.
pub(crate) mod csr_admin_user_row {
    pub use crate::csr_admin_user_row::*;
}
