pub(super) fn csr_admin_role_row(
    item: &server_admin_contract::admin_role_summary::AdminRoleSummary,
    page: &server_admin_contract::admin_roles_page::AdminRolesPage,
) -> impl leptos::prelude::IntoView + use<> {
    let id = item.id().to_string();
    let name = item.name().to_string();
    let system = item.is_system().to_string();
    let permissions =
        crate::domain_types::shared::admin_role_permissions::admin_role_permissions(item, page);
    leptos::view! {
        <crate::table_row::TableRow>
            <crate::table_cell::TableCell data_label="id">{id}</crate::table_cell::TableCell>
            <crate::table_cell::TableCell data_label="name">{name}</crate::table_cell::TableCell>
            <crate::table_cell::TableCell data_label="system">{system}</crate::table_cell::TableCell>
            {permissions}
        </crate::table_row::TableRow>
    }
}
