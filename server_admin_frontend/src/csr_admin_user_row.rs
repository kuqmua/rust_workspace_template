pub(super) fn csr_admin_user_row(
    item: &server_admin_contract::admin_user_summary::AdminUserSummary,
    page: &server_admin_contract::admin_users_page::AdminUsersPage,
) -> impl leptos::prelude::IntoView + use<> {
    let id = item.id().to_string();
    let login = item.login().to_string();
    let display_name = item.display_name().to_string();
    let banned = item.is_banned().to_string();
    let roles = crate::domain_types::shared::admin_user_roles::admin_user_roles(item, page);
    leptos::view! {
        <crate::table_row::TableRow>
            <crate::table_cell::TableCell data_label="id">{id}</crate::table_cell::TableCell>
            <crate::table_cell::TableCell data_label="login">{login}</crate::table_cell::TableCell>
            <crate::table_cell::TableCell data_label="display_name">{display_name}</crate::table_cell::TableCell>
            <crate::table_cell::TableCell data_label="banned">{banned}</crate::table_cell::TableCell>
            {roles}
        </crate::table_row::TableRow>
    }
}
