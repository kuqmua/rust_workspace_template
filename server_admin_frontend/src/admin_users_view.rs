use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
pub(crate) fn AdminUsersView(
    authenticated_admin: server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_users_page: server_admin_contract::admin_users_page::AdminUsersPage,
    admin_csr_query: super::admin_csr_query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
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
    let total = admin_users_page.total();
    let rows = admin_users_page
        .items()
        .iter()
        .map(|item| crate::csr_admin_user_row::csr_admin_user_row(item, &admin_users_page))
        .collect::<Vec<_>>();
    leptos::view! {
        <section class="table-admin_users_page" data-renderer="csr">
            <div class="resource-actions">
                {can_create.then(|| leptos::view! { <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersCreate.get()>"Create user"</crate::admin_button_link::AdminButtonLink> })}
                {can_manage.then(|| leptos::view! { <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersManage.get() admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary>"Manage users"</crate::admin_button_link::AdminButtonLink> })}
            </div>
            <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead>"id"</crate::table_head::TableHead><crate::table_head::TableHead>"login"</crate::table_head::TableHead><crate::table_head::TableHead>"display_name"</crate::table_head::TableHead><crate::table_head::TableHead>"banned"</crate::table_head::TableHead><crate::table_head::TableHead>"roles"</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
            <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
            <super::admin_pagination::AdminPagination admin_frontend_path=server_admin_contract::admin_frontend_path::AdminFrontendPath::Users admin_csr_query=admin_csr_query admin_page_total=total />
        </section>
    }
}

// Root-owned module compatibility wrappers.
