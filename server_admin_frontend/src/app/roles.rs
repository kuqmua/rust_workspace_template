use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

mod actions;
mod create;
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
    let can_create = admin.has_permission(server_admin_contract::AdminPermission::RolesCreate);
    let can_delete = admin.has_permission(server_admin_contract::AdminPermission::RolesDelete);
    let can_update = admin.has_permission(server_admin_contract::AdminPermission::RolesUpdate);
    let can_update_permissions =
        admin.has_permission(server_admin_contract::AdminPermission::RolePermissionsUpdate);
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            {crate::shared::table_filters::form::admin_table_filters(server_admin_contract::AdminFrontendPath::Roles, &query.search, &query.sort, crate::shared::table_filters::form::AdminTableFilterDirection::from_csr(query.direction.as_ref()), query.limit, &server_admin_contract::AdminTableSortField::ROLE, crate::shared::table_filters::form::AdminTableFilterPresentation::Csr)}
            <create::AdminCreateRole can_create=can_create />
            <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"name"</th><th>"system"</th><th>"permissions"</th><th>"actions"</th></tr></thead>
            <tbody>{page.items().iter().map(|item| {
                row::admin_role_row(item, &page, can_delete, can_update, can_update_permissions)
            }).collect::<Vec<_>>()}</tbody></table></div>
            <p>{format!("{} total", page.total())}</p>
            <super::pagination::AdminPagination action=server_admin_contract::AdminFrontendPath::Roles query=query total=page.total() />
        </section>
    }
}
