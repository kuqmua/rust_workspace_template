use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

mod actions;
mod edit;
mod row;

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
pub(in crate::app) fn AdminRolesView(
    admin: server_admin_contract::AuthenticatedAdmin,
    page: server_admin_contract::AdminRolesPage,
    query: super::query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let can_delete = admin.has_permission(server_admin_contract::AdminPermission::RolesDelete);
    let can_update = admin.has_permission(server_admin_contract::AdminPermission::RolesUpdate);
    let can_update_permissions =
        admin.has_permission(server_admin_contract::AdminPermission::RolePermissionsUpdate);
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"name"</th><th>"system"</th><th>"permissions"</th><th>"actions"</th></tr></thead>
            <tbody>{page.items().iter().map(|item| {
                row::admin_role_row(item, &page, can_delete, can_update, can_update_permissions)
            }).collect::<Vec<_>>()}</tbody></table></div>
            <super::pagination::AdminPagination action=server_admin_contract::AdminFrontendPath::Roles query=query total=page.total() />
        </section>
    }
}
