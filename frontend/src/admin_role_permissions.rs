#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(crate) fn admin_role_permissions(
    admin_role_summary: &server_admin_contract::admin_role_summary::AdminRoleSummary,
    admin_roles_page: &server_admin_contract::admin_roles_page::AdminRolesPage,
) -> impl leptos::prelude::IntoView + use<> {
    let names = String::from(crate::join_text::join_text(
        admin_roles_page
            .permissions()
            .iter()
            .filter(|permission| {
                admin_role_summary
                    .permission_ids()
                    .contains(&permission.id())
            })
            .map(server_admin_contract::admin_permission_summary::AdminPermissionSummary::name)
            .map(|name| name.as_ref().as_str()),
    ));
    leptos::view! { <crate::table_cell::TableCell data_label="permissions">{names}</crate::table_cell::TableCell> }
}
