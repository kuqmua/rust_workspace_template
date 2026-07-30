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
pub(in crate::app) fn AdminUsersView(
    admin: server_admin_contract::AuthenticatedAdmin,
    page: server_admin_contract::AdminUsersPage,
    query: super::query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let can_create = admin.has_permission(server_admin_contract::AdminPermission::UsersCreate);
    let can_delete = admin.has_permission(server_admin_contract::AdminPermission::UsersDelete);
    let can_update = admin.has_permission(server_admin_contract::AdminPermission::UsersUpdate);
    let can_update_roles =
        admin.has_permission(server_admin_contract::AdminPermission::UserRolesUpdate);
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            {crate::shared::table_filters::form::admin_table_filters(server_admin_contract::AdminFrontendPath::Users, &query.search, &query.sort, crate::shared::table_filters::form::AdminTableFilterDirection::from_csr(query.direction.as_ref()), query.limit, &server_admin_contract::AdminTableSortField::USER, crate::shared::table_filters::form::AdminTableFilterPresentation::Csr)}
            <create::AdminCreateUser can_create=can_create />
            <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"login"</th><th>"display_name"</th><th>"banned"</th><th>"roles"</th><th>"actions"</th></tr></thead>
            <tbody>{page.items().iter().map(|item| {
                row::admin_user_row(item, &page, can_delete, can_update, can_update_roles)
            }).collect::<Vec<_>>()}</tbody></table></div>
            <p>{format!("{} total", page.total())}</p>
            <super::pagination::AdminPagination action=server_admin_contract::AdminFrontendPath::Users query=query total=page.total() />
        </section>
    }
}
