pub(super) fn csr_admin_role_row(
    admin_role_summary: &server_admin_contract::admin_role_summary::AdminRoleSummary,
    admin_roles_page: &server_admin_contract::admin_roles_page::AdminRolesPage,
) -> impl leptos::prelude::IntoView + use<> {
    let id = admin_role_summary.id().to_string();
    let name = admin_role_summary.name().to_string();
    let system = admin_role_summary.is_system().to_string();
    let permissions = crate::domain_types::shared::admin_role_permissions::admin_role_permissions(
        admin_role_summary,
        admin_roles_page,
    );
    leptos::view! {
        <crate::table_row::TableRow>
            <crate::table_cell::TableCell data_label="id">{id}</crate::table_cell::TableCell>
            <crate::table_cell::TableCell data_label="name">{name}</crate::table_cell::TableCell>
            <crate::table_cell::TableCell data_label="system">{system}</crate::table_cell::TableCell>
            {permissions}
        </crate::table_row::TableRow>
    }
}
