use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

mod actions;
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
    let can_delete = admin.has_permission(server_admin_contract::AdminPermission::UsersDelete);
    let can_update = admin.has_permission(server_admin_contract::AdminPermission::UsersUpdate);
    let can_update_roles =
        admin.has_permission(server_admin_contract::AdminPermission::UserRolesUpdate);
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"login"</th><th>"display_name"</th><th>"banned"</th><th>"roles"</th><th>"actions"</th></tr></thead>
            <tbody>{page.items().iter().map(|item| {
                row::admin_user_row(item, &page, can_delete, can_update, can_update_roles)
            }).collect::<Vec<_>>()}</tbody></table></div>
            <super::pagination::AdminPagination action=server_admin_contract::AdminFrontendPath::Users query=query total=page.total() />
        </section>
    }
}
