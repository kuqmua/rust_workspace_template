pub(super) fn csr_admin_user_row(
    admin_user_summary: &server_admin_contract::admin_user_summary::AdminUserSummary,
    admin_users_page: &server_admin_contract::admin_users_page::AdminUsersPage,
) -> impl leptos::prelude::IntoView + use<> {
    let id = admin_user_summary.id().to_string();
    let login = admin_user_summary.login().to_string();
    let display_name = admin_user_summary.display_name().to_string();
    let banned = admin_user_summary.is_banned().to_string();
    let roles = crate::admin_user_roles::admin_user_roles(admin_user_summary, admin_users_page);
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
