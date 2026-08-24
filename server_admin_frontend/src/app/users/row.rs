pub(super) fn admin_user_row(
    item: &server_admin_contract::AdminUserSummary,
    page: &server_admin_contract::AdminUsersPage,
) -> impl leptos::prelude::IntoView + use<> {
    let id = item.id().to_string();
    let login = item.login().to_string();
    let display_name = item.display_name().to_string();
    let banned = item.is_banned().to_string();
    let roles = crate::shared::admin_table_cells::admin_user_roles(item, page);
    leptos::view! {
        <crate::ui::table::TableRow>
            <crate::ui::table::TableCell data_label="id">{id}</crate::ui::table::TableCell>
            <crate::ui::table::TableCell data_label="login">{login}</crate::ui::table::TableCell>
            <crate::ui::table::TableCell data_label="display_name">{display_name}</crate::ui::table::TableCell>
            <crate::ui::table::TableCell data_label="banned">{banned}</crate::ui::table::TableCell>
            {roles}
        </crate::ui::table::TableRow>
    }
}
