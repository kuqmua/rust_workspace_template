use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
pub(crate) fn AdminRolesView(
    admin: server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    page: server_admin_contract::admin_roles_page::AdminRolesPage,
    query: super::admin_csr_query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let can_create = bool::from(
        admin.has_permission(server_admin_contract::admin_permission::AdminPermission::RolesCreate),
    );
    let can_manage = bool::from(
        admin.has_permission(server_admin_contract::admin_permission::AdminPermission::RolesUpdate),
    ) || bool::from(
        admin.has_permission(server_admin_contract::admin_permission::AdminPermission::RolesDelete),
    );
    let total = page.total();
    let rows = page
        .items()
        .iter()
        .map(|item| csr_admin_role_row::csr_admin_role_row(item, &page))
        .collect::<Vec<_>>();
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <div class="resource-actions">
                {can_create.then(|| leptos::view! { <crate::admin_button_link::AdminButtonLink href=server_admin_contract::admin_frontend_path::AdminFrontendPath::RolesCreate.get()>"Create role"</crate::admin_button_link::AdminButtonLink> })}
                {can_manage.then(|| leptos::view! { <crate::admin_button_link::AdminButtonLink href=server_admin_contract::admin_frontend_path::AdminFrontendPath::RolesManage.get() variant=crate::admin_button_variant::AdminButtonVariant::Secondary>"Manage roles"</crate::admin_button_link::AdminButtonLink> })}
            </div>
            <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead>"id"</crate::table_head::TableHead><crate::table_head::TableHead>"name"</crate::table_head::TableHead><crate::table_head::TableHead>"system"</crate::table_head::TableHead><crate::table_head::TableHead>"permissions"</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
            <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
            <super::admin_pagination::AdminPagination action=server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles query=query total=total />
        </section>
    }
}

// Root-owned module compatibility wrappers.
pub(crate) mod csr_admin_role_row {}
